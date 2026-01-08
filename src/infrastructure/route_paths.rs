/* ------------------------------------------------------------------------------------------------
 * Fyn-Front: Modern CFD/CAE Web Interface
 * Copyright (C) 2025 Fyn-Front Authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 * ------------------------------------------------------------------------------------------------
 * filename: routing.rs
 * description: Application routing paths and their 'names'.
 * ------------------------------------------------------------------------------------------------
 */

pub struct RoutePath {
    pub path: &'static str,
    pub name: &'static str,
}

pub const HOME: RoutePath = RoutePath {
    path: "/",
    name: "Home",
};

pub const SHOWCASE: RoutePath = RoutePath {
    path: "/showcase",
    name: "Showcase",
};

pub const SIGN_IN: RoutePath = RoutePath {
    path: "/sign_in",
    name: "Sign In",
};

pub const SIMULATE: RoutePath = RoutePath {
    path: "/simulate",
    name: "Simulate",
};

pub const REGISTER: RoutePath = RoutePath {
    path: "/register",
    name: "Register",
};

pub const USER_PREFERENCES: RoutePath = RoutePath {
    path: "/user_preferences",
    name: "User Preferences",
};
