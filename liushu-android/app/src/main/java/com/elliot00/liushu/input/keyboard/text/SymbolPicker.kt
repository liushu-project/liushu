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

package com.elliot00.liushu.input.keyboard.text

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowLeft
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.FilledTonalButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.elliot00.liushu.input.keyboard.KeyCode
import kotlinx.coroutines.launch

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun SymbolPicker(symbolsData: Array<Pair<String, Array<String>>>, onKeyPressed: (KeyCode) -> Unit) {
    val pagerState = rememberPagerState(initialPage = 0)

    Column {
        HorizontalPager(state = pagerState, pageCount = symbolsData.size) { page ->
            LazyVerticalGrid(
                columns = GridCells.Adaptive(minSize = 50.dp),
                modifier = Modifier.height(260.dp)
            ) {
                items(symbolsData[page].second) { symbol ->
                    TextButton(onClick = { onKeyPressed(KeyCode.RawText(symbol)) }) {
                        Text(text = symbol, fontSize = 24.sp)
                    }
                }
            }
        }

        val coroutineScope = rememberCoroutineScope()
        LazyRow(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.Center) {
            item {
                IconButton(onClick = { onKeyPressed(KeyCode.Abc) }) {
                    Icon(Icons.Filled.KeyboardArrowLeft, "goBackToQwerty")
                }
            }
            itemsIndexed(symbolsData) { index, (category, _) ->
                val color =
                    if (pagerState.currentPage == index) Color.LightGray else MaterialTheme.colorScheme.background
                FilledTonalButton(
                    onClick = { coroutineScope.launch { pagerState.scrollToPage(index) } },
                    colors = ButtonDefaults.filledTonalButtonColors(containerColor = color)
                ) {
                    Text(category, fontSize = 24.sp)
                }
            }
        }
    }
}