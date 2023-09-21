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

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import com.elliot00.liushu.R

@Composable
fun QwertyKeyboard(onKeyPressed: (KeyCode) -> Unit) {
    Column(Modifier.padding(4.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
        keysMatrix.forEach { row ->
            Row(
                Modifier
                    .fillMaxWidth()
                    .height(42.dp),
                horizontalArrangement = Arrangement.spacedBy(
                    4.dp,
                    Alignment.CenterHorizontally
                ),
                verticalAlignment = Alignment.CenterVertically
            ) {
                row.forEach { data ->
                    val modifier: Modifier = Modifier.fillMaxHeight()

                    when (data.keyCode) {
                        is KeyCode.Enter -> {
                            Row(
                                modifier = modifier
                                    .background(
                                        color = MaterialTheme.colorScheme.tertiaryContainer,
                                        shape = MaterialTheme.shapes.extraLarge
                                    )
                                    .clip(shape = MaterialTheme.shapes.extraLarge)
                                    .weight(1.5f)
                                    .clickable {
                                        onKeyPressed(data.keyCode)
                                    },
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {
                                Icon(
                                    painterResource(id = R.drawable.ic_baseline_keyboard_return_24),
                                    contentDescription = "return",
                                    tint = MaterialTheme.colorScheme.onTertiaryContainer
                                )
                            }
                        }

                        is KeyCode.Shift -> {
                            Row(
                                modifier = modifier
                                    .background(
                                        color = MaterialTheme.colorScheme.primaryContainer,
                                        shape = MaterialTheme.shapes.medium
                                    )
                                    .clip(shape = MaterialTheme.shapes.medium)
                                    .weight(1.5f)
                                    .clickable {
                                        onKeyPressed(data.keyCode)
                                    },
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {
                                Icon(
                                    painter = painterResource(id = R.drawable.ic_capslock_none),
                                    contentDescription = "shift",
                                    tint = MaterialTheme.colorScheme.onPrimaryContainer
                                )
                            }
                        }

                        is KeyCode.Delete -> {
                            Row(
                                modifier = modifier
                                    .background(
                                        color = MaterialTheme.colorScheme.primaryContainer,
                                        shape = MaterialTheme.shapes.medium
                                    )
                                    .clip(shape = MaterialTheme.shapes.medium)
                                    .weight(1.5f)
                                    .clickable {
                                        onKeyPressed(data.keyCode)
                                    },
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {
                                Icon(
                                    painter = painterResource(id = R.drawable.ic_outline_backspace_24),
                                    contentDescription = "delete",
                                    tint = MaterialTheme.colorScheme.onPrimaryContainer
                                )
                            }
                        }

                        is KeyCode.Comma, is KeyCode.Period -> {
                            Row(
                                modifier = modifier
                                    .background(
                                        color = MaterialTheme.colorScheme.primaryContainer,
                                        shape = MaterialTheme.shapes.medium
                                    )
                                    .clip(shape = MaterialTheme.shapes.medium)
                                    .weight(1f)
                                    .clickable {
                                        onKeyPressed(data.keyCode)
                                    },
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {
                                Text(
                                    text = data.label,
                                    style = MaterialTheme.typography.titleLarge,
                                    color = MaterialTheme.colorScheme.onPrimaryContainer
                                )
                            }
                        }

                        is KeyCode.Symbols -> {
                            Row(
                                modifier = modifier
                                    .background(
                                        color = MaterialTheme.colorScheme.secondaryContainer,
                                        shape = MaterialTheme.shapes.extraLarge
                                    )
                                    .clip(shape = MaterialTheme.shapes.extraLarge)
                                    .weight(1.5f)
                                    .clickable {
                                        onKeyPressed(data.keyCode)
                                    },
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {
                                Text(
                                    text = data.label,
                                    color = MaterialTheme.colorScheme.onSecondaryContainer
                                )
                            }
                        }

                        is KeyCode.Space -> {
                            Row(
                                modifier = modifier
                                    .background(
                                        color = MaterialTheme.colorScheme.surface,
                                        shape = MaterialTheme.shapes.medium
                                    )
                                    .clip(shape = MaterialTheme.shapes.medium)
                                    .weight(4f)
                                    .clickable {
                                        onKeyPressed(data.keyCode)
                                    },
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {}
                        }

                        is KeyCode.Emoji -> {
                            Row(
                                modifier = modifier
                                    .background(
                                        color = MaterialTheme.colorScheme.surface,
                                        shape = MaterialTheme.shapes.medium
                                    )
                                    .clip(shape = MaterialTheme.shapes.medium)
                                    .weight(1f)
                                    .clickable {
                                        onKeyPressed(data.keyCode)
                                    },
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {
                                Icon(
                                    painter = painterResource(id = R.drawable.ic_outline_tag_faces_24),
                                    contentDescription = "emoji",
                                    tint = MaterialTheme.colorScheme.onSurface
                                )
                            }
                        }

                        else -> {
                            Row(
                                modifier = modifier
                                    .background(
                                        color = MaterialTheme.colorScheme.surface,
                                        shape = MaterialTheme.shapes.medium
                                    )
                                    .clip(shape = MaterialTheme.shapes.medium)
                                    .weight(1f)
                                    .clickable {
                                        onKeyPressed(data.keyCode)
                                    },
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.Center
                            ) {
                                Text(
                                    text = data.label,
                                    style = MaterialTheme.typography.titleLarge,
                                    color = MaterialTheme.colorScheme.onSurface
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

private val keysMatrix = arrayOf(
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
        KeyData(label = "shift", keyCode = KeyCode.Shift),
        KeyData(label = "z", keyCode = KeyCode.Alpha("z")),
        KeyData(label = "x", keyCode = KeyCode.Alpha("x")),
        KeyData(label = "c", keyCode = KeyCode.Alpha("c")),
        KeyData(label = "v", keyCode = KeyCode.Alpha("v")),
        KeyData(label = "b", keyCode = KeyCode.Alpha("b")),
        KeyData(label = "n", keyCode = KeyCode.Alpha("n")),
        KeyData(label = "m", keyCode = KeyCode.Alpha("m")),
        KeyData(label = "删除", keyCode = KeyCode.Delete),
    ),
    arrayOf(
        KeyData(label = "?123", keyCode = KeyCode.Symbols),
        KeyData(label = "，", keyCode = KeyCode.Comma),
        KeyData(label = ":>", keyCode = KeyCode.Emoji),
        KeyData(label = "　", keyCode = KeyCode.Space),
        KeyData(label = "。", keyCode = KeyCode.Period),
        KeyData(label = "回车", keyCode = KeyCode.Enter)
    )
)