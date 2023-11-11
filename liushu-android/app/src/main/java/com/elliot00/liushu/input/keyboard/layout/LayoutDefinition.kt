/*
 * Copyright (C) 2023  Elliot Xu
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

package com.elliot00.liushu.input.keyboard.layout.preset

import com.elliot00.liushu.input.keyboard.key.KeyDefinition

data class LayoutDefinition(val keyMatrix: Array<Array<KeyboardItemDef>>) {
    sealed class KeyboardItemDef {
        data class KeyDef(val definition: KeyDefinition) : KeyboardItemDef()
        data class Placeholder(val widthWeight: Float) : KeyboardItemDef()
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as LayoutDefinition

        if (!keyMatrix.contentDeepEquals(other.keyMatrix)) return false

        return true
    }

    override fun hashCode(): Int {
        return keyMatrix.contentDeepHashCode()
    }
}