/* 
* This file is part of Fyn-Front.
*
* Copyright (C) 2025 Bevan Jones
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, version 3.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*
* File: api-service.test.js
* Description: tests api-service.js.
*/

import { handleResponse } from '../../src/utilities/api-service';

describe('handleResponse', () => {
  it('should parse successful JSON response', async () => {
    const mockResponse = {
      ok: true,
      json: jest.fn().mockResolvedValue({ data: 'test' }),
      text: jest.fn()
    };

    const result = await handleResponse(mockResponse);
    expect(result).toEqual({ data: 'test' });
    expect(mockResponse.json).toHaveBeenCalledTimes(1);
    expect(mockResponse.text).not.toHaveBeenCalled();
  });

  it('should throw error for unsuccessful response', async () => {
    const mockResponse = {
      ok: false,
      status: 404,
      text: jest.fn().mockResolvedValue('Not Found'),
      json: jest.fn()
    };

    await expect(handleResponse(mockResponse)).rejects.toThrow('Something went wrong');
    expect(mockResponse.text).toHaveBeenCalledTimes(1);
  });
});