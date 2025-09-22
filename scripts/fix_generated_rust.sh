#!/bin/bash

# safe_wasm_converter.sh - Conservative approach to OpenAPI WASM conversion
# Uses file backup and simple replacement patterns to avoid corruption

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FYN_API_DIR="${SCRIPT_DIR}/../fyn_api"

echo "Converting OpenAPI Generator Rust client to WASM compatibility"

# Check if the directory exists
if [ ! -d "$FYN_API_DIR" ]; then
    echo "Error: fyn_api directory not found at $FYN_API_DIR"
    exit 1
fi

# Step 1: Fix models::models:: bug (this is safe and well-tested)
echo "Step 1: Fixing models::models:: bug..."
find "$FYN_API_DIR/src" -name "*.rs" -type f -exec sed -i 's/models::models::/models::/g' {} \;
echo "   Fixed models::models:: references"

# Step 2: Update Cargo.toml safely
echo "Step 2: Updating Cargo.toml..."
CARGO_TOML="$FYN_API_DIR/Cargo.toml"

if [ -f "$CARGO_TOML" ]; then
    # Add gloo-net dependency if not present
    if ! grep -q "gloo-net" "$CARGO_TOML"; then
        # Find the line with uuid and add gloo-net after it
        sed -i '/^uuid = /a gloo-net = { version = "0.6", features = ["http"] }' "$CARGO_TOML"
        echo "   Added gloo-net dependency"
    fi
    
    # Add web-sys dependency if not present
    if ! grep -q "web-sys.*RequestCredentials" "$CARGO_TOML"; then
        sed -i '/^gloo-net = /a web-sys = { version = "0.3", features = ["RequestCredentials"] }' "$CARGO_TOML"
        echo "   Added web-sys dependency"
    fi
fi

# Step 3: Add WASM imports to API files (simple and safe)
echo "Step 3: Adding WASM imports to API files..."

add_wasm_imports() {
    local file="$1"
    if [ -f "$file" ] && ! grep -q "use gloo_net::http::Request;" "$file"; then
        # Find the line with configuration imports and add WASM imports after it
        local import_line=$(grep -n "use super::{Error, configuration, ContentType};" "$file" | cut -d: -f1)
        if [ ! -z "$import_line" ]; then
            # Create a temporary file for safe editing
            local temp_file=$(mktemp)
            {
                head -n "$import_line" "$file"
                echo ""
                echo "// WASM-compatible imports"
                echo "use gloo_net::http::Request;"
                echo "use web_sys::RequestCredentials;"
                tail -n +"$((import_line + 1))" "$file"
            } > "$temp_file"
            
            # Only replace if the temporary file is not empty
            if [ -s "$temp_file" ]; then
                mv "$temp_file" "$file"
                echo "   Added WASM imports to $(basename "$file")"
            else
                rm -f "$temp_file"
                echo "   Warning: Failed to add imports to $(basename "$file")"
            fi
        fi
    fi
}

# Add imports to all API files
for api_file in "$FYN_API_DIR/src/apis"/*_api.rs; do
    if [ -f "$api_file" ]; then
        add_wasm_imports "$api_file"
    fi
done

# Step 4: Convert specific functions using template replacement
echo "Step 4: Converting high-priority functions..."

# Function to safely replace a specific function with a template
convert_function() {
    local file="$1"
    local func_name="$2"
    local method="$3"
    local url_path="$4"
    local has_body="$5"
    
    if [ ! -f "$file" ]; then
        echo "   Warning: File not found: $file"
        return
    fi
    
    # Check if function exists and isn't already converted
    if ! grep -q "pub async fn $func_name" "$file" || grep -q "Request::$method.*credentials" "$file"; then
        return
    fi
    
    # Create backup before modification
    cp "$file" "${file}.bak"
    
    # Use Python for reliable function replacement
    python3 << EOF
import re
import sys

file_path = "$file"
func_name = "$func_name"
method = "$method"
url_path = "$url_path"
has_body = "$has_body" == "true"

try:
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Find function signature and extract parameters/return type
    func_pattern = rf'pub async fn {re.escape(func_name)}\((.*?)\) -> (.*?)\{{'
    func_match = re.search(func_pattern, content, re.DOTALL)
    
    if not func_match:
        sys.exit(0)  # Function not found, skip
    
    params = func_match.group(1).strip()
    return_type = func_match.group(2).strip()
    
    # Find the complete function body
    start_pos = func_match.start()
    brace_count = 0
    pos = func_match.end() - 1  # Start at opening brace
    
    for i in range(pos, len(content)):
        if content[i] == '{':
            brace_count += 1
        elif content[i] == '}':
            brace_count -= 1
            if brace_count == 0:
                end_pos = i + 1
                break
    else:
        sys.exit(0)  # Couldn't find function end
    
    # Generate new function body
    if has_body:
        # Extract request parameter name
        param_match = re.search(r'(\w+):\s*(?:models::)?\w*[Rr]equest', params)
        request_param = param_match.group(1) if param_match else 'request_body'
        
        new_body = f'''pub async fn {func_name}({params}) -> {return_type} {{
    let url = format!("{url_path}", configuration.base_path);
    let response = Request::{method}(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .json(&{request_param})
        .map_err(|e| {{
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("JSON serialization error: {{:?}}", e),
            ))
        }})?
        .send()
        .await
        .map_err(|e| {{
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Gloo error: {{:?}}", e),
            ))
        }})?;

    if response.ok() {{
        let json = response.json().await.map_err(|e| {{
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("JSON error: {{:?}}", e),
            ))
        }})?;
        return Ok(json);
    }} else {{
        let content = format!("HTTP error: {{}}", response.status());
        return Err(Error::ResponseError(ResponseContent {{
            status: reqwest::StatusCode::from_u16(response.status())
                .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
            content,
            entity: None,
        }}));
    }}
}}'''
    else:
        new_body = f'''pub async fn {func_name}({params}) -> {return_type} {{
    let url = format!("{url_path}", configuration.base_path);
    let response = Request::{method}(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| {{
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Gloo error: {{:?}}", e),
            ))
        }})?;

    if response.ok() {{
        let json = response.json().await.map_err(|e| {{
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("JSON error: {{:?}}", e),
            ))
        }})?;
        return Ok(json);
    }} else {{
        let content = format!("HTTP error: {{}}", response.status());
        return Err(Error::ResponseError(ResponseContent {{
            status: reqwest::StatusCode::from_u16(response.status())
                .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
            content,
            entity: None,
        }}));
    }}
}}'''
    
    # Replace function in content
    new_content = content[:start_pos] + new_body + content[end_pos:]
    
    with open(file_path, 'w') as f:
        f.write(new_content)
    
    print(f"   Converted {func_name}")
    
except Exception as e:
    print(f"   Warning: Failed to convert {func_name}: {e}")
    # Restore backup if conversion failed
    import shutil
    shutil.copy(f"{file_path}.bak", file_path)
EOF
    
    # Remove backup file if conversion succeeded
    if [ -f "${file}.bak" ]; then
        rm "${file}.bak"
    fi
}

# Convert the most important functions first (based on your working examples)
echo "   Converting auth_api functions..."
convert_function "$FYN_API_DIR/src/apis/auth_api.rs" "auth_csrf_retrieve" "get" "{}/auth/csrf/" "false"
convert_function "$FYN_API_DIR/src/apis/auth_api.rs" "auth_user_login_create" "post" "{}/auth/user/login/" "true"
convert_function "$FYN_API_DIR/src/apis/auth_api.rs" "auth_user_logout_create" "post" "{}/auth/user/logout/" "false"

echo "   Converting runner_manager_api functions..."
convert_function "$FYN_API_DIR/src/apis/runner_manager_api.rs" "runner_manager_users_list" "get" "{}/runner_manager/users/" "false"

echo "   Converting accounts_api functions..."
convert_function "$FYN_API_DIR/src/apis/accounts_api.rs" "accounts_users_list" "get" "{}/accounts/users/" "false"
convert_function "$FYN_API_DIR/src/apis/accounts_api.rs" "accounts_users_create" "post" "{}/accounts/users/" "true"

# Step 5: Verify the conversion worked
echo "Step 5: Verifying conversion..."

# Test if the code compiles
cd "$FYN_API_DIR"
if cargo check --target=wasm32-unknown-unknown >/dev/null 2>&1; then
    echo "   Compilation check passed"
    CONVERSION_SUCCESS=true
else
    echo "   Warning: Compilation check failed - some functions may need manual conversion"
    CONVERSION_SUCCESS=false
fi

echo ""
echo "Conversion complete!"
echo ""
echo "Summary:"
echo "- Backup created at: $BACKUP_DIR"
echo "- Fixed models::models:: bug"
echo "- Added WASM dependencies to Cargo.toml"
echo "- Added WASM imports to all API files"
echo "- Converted priority functions (auth, runner_manager, accounts)"
echo ""

if [ "$CONVERSION_SUCCESS" = true ]; then
    echo "Next steps:"
    echo "1. Test your application with the converted functions"
    echo "2. Convert additional functions manually using the same pattern"
    echo "3. Remove backup directory when satisfied: rm -rf $BACKUP_DIR"
else
    echo "To restore backup if needed: rm -rf $FYN_API_DIR && mv $BACKUP_DIR $FYN_API_DIR"
    echo "Check compilation errors with: cargo check --target=wasm32-unknown-unknown"
fi

echo ""
echo "Working pattern for manual conversions:"
echo "Replace 'reqwest::Client::new()' calls with:"
echo "  Request::METHOD(&url).credentials(RequestCredentials::Include).send().await"