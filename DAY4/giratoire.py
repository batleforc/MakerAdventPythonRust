# main.py -- put your code here!
from machine import ADC, Pin
import time

green = Pin(18, Pin.OUT)
amber = Pin(19, Pin.OUT)
red = Pin(20, Pin.OUT)

potentiometer = ADC(Pin(27))

mydelay = 0

reader = 0

while True:
  mydelay = potentiometer.read_u16()

  if mydelay <= 20000: # If reading is less than or equal to 20000
    red.value(1) # Red ON
    amber.value(0)
    green.value(0)
  elif 20000 < mydelay < 40000: # If reading is between 20000 and 40000
    red.value(0)
    amber.value(1) # Amber ON
    green.value(0)
  elif mydelay >= 40000: # If reading is greater than or equal to 40000
    red.value(0)
    amber.value(0)
    green.value(1) # Green ON