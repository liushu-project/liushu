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

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.background
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.res.painterResource

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun RowScope.Key(
    appearance: KeyDefinition.Appearance,
    onClick: () -> Unit,
    onLongClick: () -> Unit,
    modifier: Modifier = Modifier,
    showAsciiText: Boolean = false,
    showCapsLockText: Boolean = false
) {
    Row(
        modifier = modifier
            .fillMaxHeight()
            .background(
                color = appearance.backgroundColor, shape = appearance.shape
            )
            .clip(shape = appearance.shape)
            .weight(appearance.widthWeight)
            .combinedClickable(
                onClick = onClick, onLongClick = onLongClick
            ),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.Center
    ) {
        KeyLabel(label = appearance.label, showAsciiText, showCapsLockText)
    }
}

@Composable
private fun KeyLabel(
    label: KeyDefinition.Appearance.Label,
    showAsciiText: Boolean = false,
    showCapsLockText: Boolean = false
) {
    when (label) {
        is KeyDefinition.Appearance.Label.TextLabel -> {
            val text = if (showCapsLockText) {
                label.textInCapsLock
            } else if (showAsciiText) {
                label.textInAscii
            } else {
                label.text
            }

            Text(text = text, style = label.style, color = label.color)
        }

        is KeyDefinition.Appearance.Label.IconLabel -> {
            Icon(
                painter = painterResource(id = label.id),
                contentDescription = label.description,
                tint = label.color
            )
        }
    }
}