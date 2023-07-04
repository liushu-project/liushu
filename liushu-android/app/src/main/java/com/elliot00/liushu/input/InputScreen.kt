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
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.Layout
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.elliot00.liushu.service.ImeService
import com.elliot00.liushu.uniffi.Candidate

@Preview
@Composable
fun InputScreen() {
    val ctx = LocalContext.current
    val inputState = rememberInputState(context = ctx)

    val keysMatrix = arrayOf(
        arrayOf("q", "w", "e", "r", "t", "y", "u", "i", "o", "p"),
        arrayOf("a", "s", "d", "f", "g", "h", "j", "k", "l"),
        arrayOf("z", "x", "c", "v", "b", "n", "m")
    )

    Column(
        modifier = Modifier
            .background(Color(0xFF9575CD))
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
                Row(Modifier) {
                    row.forEach { key ->
                        KeyboardKey(keyboardKey = key, modifier = Modifier.weight(1f)) {
                            inputState.handleKey(it)
                        }
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

@Composable
fun KeyboardKey(
    keyboardKey: String,
    modifier: Modifier,
    onKeyPressed: (String) -> Unit
) {
    val interactionSource = remember { MutableInteractionSource() }
    val pressed = interactionSource.collectIsPressedAsState()
    Box(modifier = modifier.fillMaxHeight(), contentAlignment = Alignment.BottomCenter) {
        Text(
            keyboardKey,
            Modifier
                .fillMaxWidth()
                .padding(2.dp)
                .border(1.dp, Color.Black)
                .clickable(interactionSource = interactionSource, indication = null) {
                    onKeyPressed(keyboardKey)
                }
                .background(Color.White)
                .padding(
                    start = 12.dp,
                    end = 12.dp,
                    top = 16.dp,
                    bottom = 16.dp
                )

        )
        if (pressed.value) {
            Text(
                keyboardKey,
                Modifier
                    .fillMaxWidth()
                    .border(1.dp, Color.Black)
                    .background(Color.White)
                    .padding(
                        start = 16.dp,
                        end = 16.dp,
                        top = 16.dp,
                        bottom = 48.dp
                    )
            )
        }
    }
}

class InputStateHolder(private val context: Context) {
    private var input by mutableStateOf("")

    var candidates by mutableStateOf(listOf<Candidate>())
        private set

    fun handleKey(key: String) {
        input += key
        candidates = (context as ImeService).engine.search(input)
    }

    fun commitCandidate(candidate: Candidate) {
        (context as ImeService).currentInputConnection.commitText(
            candidate.text,
            candidate.text.length
        )
        input = ""
        candidates = listOf()
    }
}

@Composable
fun rememberInputState(context: Context): InputStateHolder =
    remember(context) {
        InputStateHolder(context)
    }