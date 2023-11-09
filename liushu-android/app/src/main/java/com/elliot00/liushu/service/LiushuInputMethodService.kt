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

package com.elliot00.liushu.service

import android.view.KeyEvent
import android.view.View
import android.view.inputmethod.EditorInfo
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.AbstractComposeView
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.ViewModelStore
import androidx.lifecycle.ViewModelStoreOwner
import androidx.lifecycle.setViewTreeLifecycleOwner
import androidx.lifecycle.setViewTreeViewModelStoreOwner
import androidx.savedstate.SavedStateRegistry
import androidx.savedstate.SavedStateRegistryController
import androidx.savedstate.SavedStateRegistryOwner
import androidx.savedstate.setViewTreeSavedStateRegistryOwner
import com.elliot00.liushu.input.InputScreen
import com.elliot00.liushu.ui.theme.LiushuTheme
import com.elliot00.liushu.uniffi.Candidate
import com.elliot00.liushu.uniffi.Engine
import java.io.File
import java.lang.ref.WeakReference

interface LiushuInputMethodServiceImpl {
    fun commitText(text: String)
    fun search(code: String): List<Candidate>
    fun handleEnter()
    fun handleDelete()
}

var ImeWeakReference = WeakReference<LiushuInputMethodServiceImpl?>(null)

class LiushuInputMethodService : LifecycleInputMethodService(), ViewModelStoreOwner,
    SavedStateRegistryOwner, LiushuInputMethodServiceImpl {
    val engine: Engine by lazy {
        val dictDir = "sunman"
        val dictFile = "sunman.trie"
        val path = sequenceOf(filesDir, dictDir, dictFile).joinToString(separator = File.separator)
        Engine(path)
    }

    override fun onCreateInputView(): View {
        val view = InputView()

        window?.window?.decorView?.let { decorView ->
            decorView.setViewTreeLifecycleOwner(this)
            decorView.setViewTreeViewModelStoreOwner(this)
            decorView.setViewTreeSavedStateRegistryOwner(this)
        }

        return view
    }

    override fun onCreate() {
        super.onCreate()
        savedStateRegistryController.performRestore(null)
        ImeWeakReference = WeakReference(this)
    }

    override fun commitText(text: String) {
        currentInputConnection.commitText(text, text.length)
    }

    override fun search(code: String): List<Candidate> {
        return engine.search(code)
    }

    override fun handleEnter() {
        currentInputConnection.performEditorAction(EditorInfo.IME_ACTION_GO)
    }

    override fun handleDelete() {
        sendDownUpKeyEvents(KeyEvent.KEYCODE_DEL)
    }

    override val viewModelStore: ViewModelStore
        get() = store
    override val lifecycle: Lifecycle
        get() = dispatcher.lifecycle


    //ViewModelStore Methods
    private val store = ViewModelStore()

    //SaveStateRegestry Methods

    private val savedStateRegistryController = SavedStateRegistryController.create(this)

    override val savedStateRegistry: SavedStateRegistry get() = savedStateRegistryController.savedStateRegistry

    private inner class InputView : AbstractComposeView(this) {
        @Composable
        override fun Content() {
            LiushuTheme {
                InputScreen()
            }
        }
    }
}

class UselessLiushuInputMethodService : LiushuInputMethodServiceImpl {
    override fun commitText(text: String) {}

    override fun search(code: String): List<Candidate> {
        return emptyList()
    }

    override fun handleEnter() {}

    override fun handleDelete() {}
}