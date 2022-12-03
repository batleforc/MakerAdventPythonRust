# boot.py -- run on boot-up
from time import sleep
from machine import Pin
onboardLED = Pin(25, Pin.OUT)
onboardLED.value(onboardLED.value() ^ 1)
print(onboardLED.value())
print("Hello World")
sleep(1)
onboardLED.value(onboardLED.value() ^ 1)