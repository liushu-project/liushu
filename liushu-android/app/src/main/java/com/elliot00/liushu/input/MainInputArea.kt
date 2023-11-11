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

package com.elliot00.liushu.input

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import com.elliot00.liushu.input.keyboard.KeyCode
import com.elliot00.liushu.input.keyboard.layout.preset.SunmanKeyboardLayout
import com.elliot00.liushu.input.picker.EmojisPicker
import com.elliot00.liushu.input.picker.SymbolsPicker

@Composable
fun MainInputArea(viewModel: InputViewModel) {
    var contentType by remember { mutableStateOf(MainInputAreaContentType.KEYBOARD) }
    val onGoBack: () -> Unit = { contentType = MainInputAreaContentType.KEYBOARD }
    val onKeyPressed: (KeyCode) -> Unit = { viewModel.handleKeyClicked(it) }

    when (contentType) {
        MainInputAreaContentType.KEYBOARD -> {
            SunmanKeyboardLayout(onMainContentTypeChange = { contentType = it }, viewModel)
        }

        MainInputAreaContentType.EMOJIS_PICKER -> {
            EmojisPicker(onKeyPressed = onKeyPressed, onGoBack = onGoBack)
        }

        MainInputAreaContentType.SYMBOLS_PICKER -> {
            SymbolsPicker(onKeyPressed = onKeyPressed, onGoBack = onGoBack)
        }
    }
}

enum class MainInputAreaContentType {
    KEYBOARD, EMOJIS_PICKER, SYMBOLS_PICKER
}