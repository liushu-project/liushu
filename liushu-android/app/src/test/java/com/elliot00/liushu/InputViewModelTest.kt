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

package com.elliot00.liushu

import com.elliot00.liushu.input.CapsLockState
import com.elliot00.liushu.input.InputViewModel
import com.elliot00.liushu.input.keyboard.KeyCode
import com.elliot00.liushu.service.LiushuInputMethodServiceImpl
import io.mockk.Runs
import io.mockk.every
import io.mockk.just
import io.mockk.mockk
import io.mockk.verify
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test

class InputViewModelTest {
    private lateinit var viewModel: InputViewModel
    private val mockIme = mockk<LiushuInputMethodServiceImpl>()

    @Before
    fun setup() {
        viewModel = InputViewModel(mockIme)
    }

    @Test
    fun inputViewModel_DeleteWithNotEmptyCandidates_InputCleared() {
        every { mockIme.search(any()) } returns emptyList()
        viewModel.handleKeyClicked(KeyCode.Alpha("a"))

        viewModel.handleKeyClicked(KeyCode.Delete)

        assertTrue(viewModel.input.value.isEmpty())
    }

    @Test
    fun inputViewModel_EnterWithEmptyInput_NoInteraction() {
        every { mockIme.handleEnter() } just Runs

        viewModel.handleKeyClicked(KeyCode.Enter)

        verify { mockIme.handleEnter() }
        verify(inverse = true) {
            mockIme.commitText(any())
        }
    }

    @Test
    fun inputViewModel_EnterWithNonEmptyInput_TextCommittedAndInputCleared() {
        every { mockIme.search(any()) } returns emptyList()
        viewModel.handleKeyClicked(KeyCode.Alpha("a"))

        every { mockIme.commitText(any()) } just Runs
        viewModel.handleKeyClicked(KeyCode.Enter)

        verify {
            mockIme.commitText("a")
        }

        assertEquals("", viewModel.input.value)
    }

    @Test
    fun inputViewModel_LongClickShift_CapsLockEnabled() {
        viewModel.handleKeyLongClicked(KeyCode.Shift)
        assertEquals(CapsLockState.ACTIVATED, viewModel.capsLockState.value)

        every { mockIme.commitText(any()) } just Runs
        viewModel.handleKeyClicked(KeyCode.Alpha("a"))

        verify { mockIme.commitText("A") }
        assertEquals(CapsLockState.ACTIVATED, viewModel.capsLockState.value)
    }

    @Test
    fun inputViewModel_ClickShift_CapsLockEnabledForSingleLetter() {
        viewModel.handleKeyClicked(KeyCode.Shift)
        assertEquals(CapsLockState.SINGLE_LETTER, viewModel.capsLockState.value)

        every { mockIme.commitText(any()) } just Runs
        viewModel.handleKeyClicked(KeyCode.Alpha("a"))

        verify { mockIme.commitText("A") }
        assertEquals(CapsLockState.DEACTIVATED, viewModel.capsLockState.value)
    }

    @Test
    fun inputViewModel_ClickShiftAfterLocked_CapsLockDisabled() {
        viewModel.handleKeyLongClicked(KeyCode.Shift)
        assertEquals(CapsLockState.ACTIVATED, viewModel.capsLockState.value)

        viewModel.handleKeyClicked(KeyCode.Shift)
        assertEquals(CapsLockState.DEACTIVATED, viewModel.capsLockState.value)
    }
}