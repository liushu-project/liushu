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

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.res.painterResource
import com.elliot00.liushu.R
import com.elliot00.liushu.input.InputViewModel

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun RowScope.Key(
    data: KeyData,
    onKeyClick: (KeyCode) -> Unit,
    onKeyLongClick: (KeyCode) -> Unit,
    modifier: Modifier = Modifier,
    viewModel: InputViewModel
) {
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
                        onKeyClick(data.keyCode)
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
                    .combinedClickable(
                        onClick = { onKeyClick(data.keyCode) },
                        onLongClick = { onKeyLongClick(data.keyCode) },
                    ),
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
                        onKeyClick(data.keyCode)
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
                        onKeyClick(data.keyCode)
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
                        onKeyClick(data.keyCode)
                    },
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center
            ) {
                Text(
                    text = data.label, color = MaterialTheme.colorScheme.onSecondaryContainer
                )
            }
        }

        is KeyCode.Space -> {
            val isAsciiMode = viewModel.isAsciiMode.collectAsState()
            Row(
                modifier = modifier
                    .background(
                        color = MaterialTheme.colorScheme.surface,
                        shape = MaterialTheme.shapes.medium
                    )
                    .clip(shape = MaterialTheme.shapes.medium)
                    .weight(4f)
                    .clickable {
                        onKeyClick(data.keyCode)
                    },
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center
            ) {
                Text(
                    text = if (isAsciiMode.value) "En" else "山人",
                    style = MaterialTheme.typography.titleLarge,
                    color = MaterialTheme.colorScheme.onSurface
                )
            }
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
                        onKeyClick(data.keyCode)
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

        is KeyCode.AsciiModeSwitch -> {
            Row(
                modifier = modifier
                    .background(
                        color = MaterialTheme.colorScheme.surface,
                        shape = MaterialTheme.shapes.medium
                    )
                    .clip(shape = MaterialTheme.shapes.medium)
                    .weight(1f)
                    .clickable {
                        onKeyClick(data.keyCode)
                    },
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center
            ) {
                Icon(
                    painter = painterResource(id = R.drawable.ic_baseline_language_24),
                    contentDescription = "language",
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
                        onKeyClick(data.keyCode)
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

data class KeyData(val label: String, val keyCode: KeyCode)