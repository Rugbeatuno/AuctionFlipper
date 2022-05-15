from random import seed
from re import I
import clipboard
import contextlib
import time
import os
from matplotlib.pyplot import contour
from pynput.keyboard import Controller, Key, Listener
from pynput.mouse import Controller as Mouse
from pynput.mouse import Button

keyboard = Controller()
mouse = Mouse()
prev_text = clipboard.paste() 

buying = 0
last_opened_auction = 0
min_time_between_auctions = 5

def click():
    mouse.click(Button.left)
    time.sleep(0.1)

def buy_routine():
    purchase_x = 721
    purchase_y = 387
    confirm_purchase_x = 650
    confirm_purchase_y = 373

    mouse.position = (purchase_x, purchase_y)
    click() 

    mouse.position = (confirm_purchase_x, confirm_purchase_y)
    click()


def on_press(key):
    global buying
    if key == Key.down:
        buying += 35
        print(f"BUY {buying}")
        buy_routine()



keyboard_listener = Listener(on_press=on_press)
keyboard_listener.start()

while True:
    time.sleep(0.03)

    if buying != 0:
        buying -= 1

    try:
        text = clipboard.paste() 
        if text != prev_text and not buying and time.time() - last_opened_auction > min_time_between_auctions:
            last_opened_auction = time.time()
            prev_text = text
            
            keyboard.press('t')
            keyboard.release('t')
            keyboard.press(Key.backspace)
            keyboard.release(Key.backspace)
            time.sleep(0.1)
            keyboard.type(text)
            time.sleep(0.1)
            keyboard.press(Key.enter)
            
    except TypeError:
        pass


