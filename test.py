import clipboard
import time
import os
from matplotlib.pyplot import contour
from pynput.keyboard import Controller, Key

keyboard = Controller()
prev_text = clipboard.paste() 
while True:
    try:
        time.sleep(0.05)
        text = clipboard.paste() 
        if text != prev_text:
            prev_text = text
            
            keyboard.press('t')
            keyboard.release('t')
            time.sleep(0.1)
            keyboard.type(text)
            time.sleep(0.1)
            keyboard.press(Key.enter)
    except TypeError:
        pass