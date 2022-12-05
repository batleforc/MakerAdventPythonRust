# main.py -- put your code here!
from machine import ADC, Pin,PWM
import time

green = PWM(Pin(18))
amber = PWM(Pin(19))
red = PWM(Pin(20))

potentiometer = ADC(Pin(27))

green.freq(1000)
amber.freq(1000)
red.freq(1000)

reader = 0

while True:
  reader = potentiometer.read_u16()

  red.duty_u16(reader)
  green.duty_u16(reader)
  amber.duty_u16(reader)
  time.sleep(0.0001)