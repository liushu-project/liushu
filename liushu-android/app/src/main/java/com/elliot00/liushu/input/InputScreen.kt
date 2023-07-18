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
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.Layout
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.elliot00.liushu.input.keyboard.Key
import com.elliot00.liushu.input.keyboard.KeyCode
import com.elliot00.liushu.input.keyboard.KeyData
import com.elliot00.liushu.service.ImeService
import com.elliot00.liushu.uniffi.Candidate

@Preview
@Composable
fun InputScreen() {
    val ctx = LocalContext.current
    val inputState = rememberInputState(context = ctx)

    val keysMatrix = arrayOf(
        arrayOf(
            KeyData(label = "q", keyCode = KeyCode.Alpha("q")),
            KeyData(label = "w", keyCode = KeyCode.Alpha("w")),
            KeyData(label = "e", keyCode = KeyCode.Alpha("e")),
            KeyData(label = "r", keyCode = KeyCode.Alpha("r")),
            KeyData(label = "t", keyCode = KeyCode.Alpha("t")),
            KeyData(label = "y", keyCode = KeyCode.Alpha("y")),
            KeyData(label = "u", keyCode = KeyCode.Alpha("u")),
            KeyData(label = "i", keyCode = KeyCode.Alpha("i")),
            KeyData(label = "o", keyCode = KeyCode.Alpha("o")),
            KeyData(label = "p", keyCode = KeyCode.Alpha("p"))
        ),
        arrayOf(
            KeyData(label = "a", keyCode = KeyCode.Alpha("a")),
            KeyData(label = "s", keyCode = KeyCode.Alpha("s")),
            KeyData(label = "d", keyCode = KeyCode.Alpha("d")),
            KeyData(label = "f", keyCode = KeyCode.Alpha("f")),
            KeyData(label = "g", keyCode = KeyCode.Alpha("g")),
            KeyData(label = "h", keyCode = KeyCode.Alpha("h")),
            KeyData(label = "j", keyCode = KeyCode.Alpha("j")),
            KeyData(label = "k", keyCode = KeyCode.Alpha("k")),
            KeyData(label = "l", keyCode = KeyCode.Alpha("l"))
        ),
        arrayOf(
            KeyData(label = "S", keyCode = KeyCode.Shift),
            KeyData(label = "z", keyCode = KeyCode.Alpha("z")),
            KeyData(label = "x", keyCode = KeyCode.Alpha("x")),
            KeyData(label = "c", keyCode = KeyCode.Alpha("c")),
            KeyData(label = "v", keyCode = KeyCode.Alpha("v")),
            KeyData(label = "b", keyCode = KeyCode.Alpha("b")),
            KeyData(label = "n", keyCode = KeyCode.Alpha("n")),
            KeyData(label = "m", keyCode = KeyCode.Alpha("m")),
            KeyData(label = "D", keyCode = KeyCode.Delete),
        )
    )

    Column(
        modifier = Modifier
            .background(MaterialTheme.colorScheme.background)
            .fillMaxWidth()
    ) {
        LazyRow(
            modifier = Modifier
                .padding(horizontal = 12.dp)
                .fillMaxWidth(),
        ) {
            items(inputState.candidates) { candidate ->
                Text(text = candidate.text, modifier = Modifier.clickable {
                    inputState.commitCandidate(candidate)
                })
                Spacer(modifier = Modifier.width(8.dp))
            }
        }
        keysMatrix.forEach { row ->
            FixedHeightBox(modifier = Modifier.fillMaxWidth(), height = 56.dp) {
                Row(
                    Modifier,
                    horizontalArrangement = Arrangement.Center,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    row.forEach { data ->
                        Key(data, onKeyPressed = { keyCode -> inputState.handleKeyCode(keyCode) })
                    }
                }
            }
        }
    }
}

@Composable
fun FixedHeightBox(modifier: Modifier, height: Dp, content: @Composable () -> Unit) {
    Layout(modifier = modifier, content = content) { measurables, constraints ->
        val placeables = measurables.map { measurable ->
            measurable.measure(constraints)
        }
        val h = height.roundToPx()
        layout(constraints.maxWidth, h) {
            placeables.forEach { placeable ->
                placeable.place(x = 0, y = kotlin.math.min(0, h - placeable.height))
            }
        }
    }
}

class InputStateHolder(private val context: Context) {
    private var input by mutableStateOf("")

    var candidates by mutableStateOf(listOf<Candidate>())
        private set

    fun handleKeyCode(keyCode: KeyCode) {
        when (keyCode) {
            is KeyCode.Alpha -> handleAlphaCode(keyCode.code)
            is KeyCode.Delete -> {
                if (input.isNotEmpty()) {
                    input = input.dropLast(1)
                    candidates = (context as ImeService).engine.search(input)
                } else {
                    (context as ImeService).currentInputConnection.deleteSurroundingText(1, 0)
                }
            }

            else -> {}
        }
    }

    fun commitCandidate(candidate: Candidate) {
        (context as ImeService).currentInputConnection.commitText(
            candidate.text,
            candidate.text.length
        )
        input = ""
        candidates = listOf()
    }

    private fun handleAlphaCode(code: String) {
        input += code
        candidates = (context as ImeService).engine.search(input)
    }
}

@Composable
fun rememberInputState(context: Context): InputStateHolder =
    remember(context) {
        InputStateHolder(context)
    }
