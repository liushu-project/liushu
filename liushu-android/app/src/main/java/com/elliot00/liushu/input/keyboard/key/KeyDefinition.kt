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

package com.elliot00.liushu.input.keyboard.key

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.text.TextStyle
import com.elliot00.liushu.input.MainInputAreaContentType
import com.elliot00.liushu.input.keyboard.KeyCode

data class KeyDefinition(
    val appearance: Appearance, val behavior: Behavior
) {
    data class Appearance(
        val label: Label, val backgroundColor: Color, val widthWeight: Float, val shape: Shape
    ) {
        sealed class Label {
            data class TextLabel(
                val text: String,
                val textInAscii: String = text,
                val textInCapsLock: String = textInAscii,
                val style: TextStyle,
                val color: Color
            ) : Label()

            data class IconLabel(val id: Int, val color: Color, val description: String?) : Label()
        }
    }

    data class Behavior(
        val clickAction: Action,
        val longClickAction: Action? = null,
    ) {
        sealed class Action {
            data class SendKeyCode(val keyCode: KeyCode) : Action()
            data class ChangeMainContent(val contentType: MainInputAreaContentType) : Action()
        }
    }
}