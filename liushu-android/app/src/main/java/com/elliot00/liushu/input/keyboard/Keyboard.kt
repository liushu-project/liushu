/*
 *     Copyright (C) 2023  Elliot Xu
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU Affero General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU Affero General Public License for more details.
 *
 *     You should have received a copy of the GNU Affero General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

package com.elliot00.liushu.input.keyboard

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import com.elliot00.liushu.input.InputViewModel
import com.elliot00.liushu.input.keyboard.text.EmojiKeyboard
import com.elliot00.liushu.input.keyboard.text.SymbolKeyboard

@Composable
fun Keyboard(viewModel: InputViewModel) {
    val layout by viewModel.keyboardLayout.collectAsState()
    val onKeyPressed: (KeyCode) -> Unit = { viewModel.handleKey(it) }

    when (layout) {
        KeyboardLayout.QWERTY -> {
            QwertyKeyboard(viewModel)
        }

        KeyboardLayout.EMOJI -> {
            EmojiKeyboard(onKeyPressed)
        }

        KeyboardLayout.SYMBOLS -> {
            SymbolKeyboard(onKeyPressed)
        }
    }

}
