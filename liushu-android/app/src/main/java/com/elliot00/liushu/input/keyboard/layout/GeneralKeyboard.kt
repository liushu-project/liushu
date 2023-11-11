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

package com.elliot00.liushu.input.keyboard.layout

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.elliot00.liushu.input.CapsLockState
import com.elliot00.liushu.input.InputViewModel
import com.elliot00.liushu.input.MainInputAreaContentType
import com.elliot00.liushu.input.keyboard.key.Key
import com.elliot00.liushu.input.keyboard.key.KeyDefinition
import com.elliot00.liushu.input.keyboard.layout.preset.LayoutDefinition

@Composable
fun GeneralKeyboard(
    layoutDef: LayoutDefinition,
    onMainContentTypeChange: (MainInputAreaContentType) -> Unit,
    viewModel: InputViewModel
) {
    Column(
        Modifier
            .padding(4.dp)
            .fillMaxHeight()
            .padding(horizontal = 3.dp, vertical = 5.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        val rowModifier = Modifier.weight(1f)

        layoutDef.keyMatrix.forEachIndexed { index, row ->
            Row(
                modifier = rowModifier.fillMaxWidth(), horizontalArrangement = Arrangement.spacedBy(
                    4.dp, Alignment.CenterHorizontally
                ), verticalAlignment = Alignment.CenterVertically
            ) {
                row.forEach { data ->
                    when (data) {
                        is LayoutDefinition.KeyboardItemDef.Placeholder -> {
                            Spacer(modifier = Modifier.weight(data.widthWeight))
                        }

                        is LayoutDefinition.KeyboardItemDef.KeyDef -> {
                            val isAsciiMode by viewModel.isAsciiMode.collectAsStateWithLifecycle()
                            val capsLockState by viewModel.capsLockState.collectAsStateWithLifecycle()
                            val onClick = when (data.definition.behavior.clickAction) {
                                is KeyDefinition.Behavior.Action.SendKeyCode -> {
                                    { viewModel.handleKeyClicked(data.definition.behavior.clickAction.keyCode) }
                                }

                                is KeyDefinition.Behavior.Action.ChangeMainContent -> {
                                    { onMainContentTypeChange(data.definition.behavior.clickAction.contentType) }
                                }
                            }
                            val onLongClick = when (data.definition.behavior.longClickAction) {
                                is KeyDefinition.Behavior.Action.SendKeyCode -> {
                                    { viewModel.handleKeyLongClicked(data.definition.behavior.longClickAction.keyCode) }
                                }

                                is KeyDefinition.Behavior.Action.ChangeMainContent -> {
                                    { onMainContentTypeChange(data.definition.behavior.longClickAction.contentType) }
                                }

                                else -> {
                                    { }
                                }
                            }
                            Key(
                                appearance = data.definition.appearance,
                                showAsciiText = isAsciiMode,
                                showCapsLockText = capsLockState == CapsLockState.ACTIVATED || capsLockState == CapsLockState.SINGLE_LETTER,
                                onClick = onClick,
                                onLongClick = onLongClick
                            )
                        }

                        else -> {}
                    }
                }

            }
        }
    }
}