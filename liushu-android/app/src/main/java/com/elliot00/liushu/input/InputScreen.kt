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

package com.elliot00.liushu.input

import android.content.Context
import android.view.KeyEvent
import android.view.inputmethod.EditorInfo
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.Divider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalConfiguration
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Popup
import com.elliot00.liushu.input.keyboard.KeyCode
import com.elliot00.liushu.input.keyboard.Keyboard
import com.elliot00.liushu.input.keyboard.KeyboardLayout
import com.elliot00.liushu.input.keyboard.candidate.CandidateItem
import com.elliot00.liushu.service.ImeService
import com.elliot00.liushu.uniffi.Candidate

@Preview
@Composable
fun InputScreen() {
    val ctx = LocalContext.current
    val inputState = rememberInputState(context = ctx)
    val screenHeight = LocalConfiguration.current.screenHeightDp.dp

    if (inputState.input.isNotEmpty()) {
        Popup(alignment = Alignment.TopStart, offset = IntOffset(0, -60)) {
            Text(
                text = inputState.input,
                modifier = Modifier
                    .background(color = MaterialTheme.colorScheme.surface)
                    .padding(horizontal = 8.dp)
            )
        }
    }
    Surface(tonalElevation = 5.dp, modifier = Modifier.height(screenHeight / 3)) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
        ) {
            LazyRow(
                modifier = Modifier
                    .height(40.dp)
                    .fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                itemsIndexed(inputState.candidates) { index, candidate ->
                    CandidateItem(
                        candidate = candidate,
                        onClick = { inputState.commitCandidate(candidate) })
                    if (index < inputState.candidates.lastIndex) {
                        Divider(
                            modifier = Modifier
                                .fillMaxHeight(0.6f)
                                .width(1.dp)
                        )
                    }
                }
            }
            Keyboard(
                onKeyPressed = { keyCode -> inputState.handleKeyCode(keyCode) },
                inputState.keyboardLayout
            )
        }
    }
}

class InputStateHolder(private val context: Context) {
    var input by mutableStateOf("")
    private var isCapital by mutableStateOf(false)

    var candidates by mutableStateOf(listOf<Candidate>())
        private set

    var keyboardLayout by mutableStateOf(KeyboardLayout.QWERTY)
        private set

    fun handleKeyCode(keyCode: KeyCode) {
        val currentInputConnection = (context as ImeService).currentInputConnection
        when (keyCode) {
            is KeyCode.Alpha -> handleAlphaCode(keyCode.code)
            is KeyCode.Delete -> {
                if (input.isNotEmpty()) {
                    input = input.dropLast(1)
                    candidates = context.engine.search(input)
                } else {
                    context.sendDownUpKeyEvents(KeyEvent.KEYCODE_DEL)
                }
            }

            is KeyCode.Comma -> {
                // TODO: shift state
                currentInputConnection.commitText("，", 1)
            }

            is KeyCode.Space -> {
                currentInputConnection.commitText("　", 1)
            }

            is KeyCode.Period -> {
                currentInputConnection.commitText("。", 1)
            }

            is KeyCode.Enter -> {
                if (input.isNotEmpty()) {
                    commitText(input)
                } else {
                    currentInputConnection.performEditorAction(EditorInfo.IME_ACTION_GO)
                }
            }

            is KeyCode.Shift -> {
                if (input.isEmpty()) {
                    isCapital = true
                }
            }

            is KeyCode.Symbols -> {
                keyboardLayout = KeyboardLayout.SYMBOLS
            }

            is KeyCode.Emoji -> {
                keyboardLayout = KeyboardLayout.EMOJI
            }

            is KeyCode.Abc -> {
                keyboardLayout = KeyboardLayout.QWERTY
            }

            is KeyCode.RawText -> {
                commitText(keyCode.text)
            }
        }
    }

    fun commitCandidate(candidate: Candidate) {
        commitText(candidate.text)
    }

    private fun commitText(text: String) {
        (context as ImeService).currentInputConnection.commitText(
            text, text.length
        )
        input = ""
        candidates = listOf()
    }

    private fun handleAlphaCode(code: String) {
        if (isCapital) {
            commitText(code.uppercase())
            isCapital = false
            return
        }

        input += code
        candidates = (context as ImeService).engine.search(input)
    }
}

@Composable
fun rememberInputState(context: Context): InputStateHolder = remember(context) {
    InputStateHolder(context)
}
