//
//  LiushuIMKInputController.swift
//  Liushu
//
//  Created by Elliot Xu on 2023/5/29.
//

import Cocoa
import InputMethodKit

@objc(LiushuIMKInputController)
class LiushuIMKInputController: IMKInputController {
    override func inputText(_ string: String!, client sender: Any!) -> Bool {
        NSLog(string)
        guard let client = sender as? IMKTextInput else {
            return false
        }
        client.insertText(string+string, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
        return true
    }
}
