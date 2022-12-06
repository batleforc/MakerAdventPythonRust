# main.py -- put your code here!
from machine import ADC, Pin,PWM
import time

buzzer = PWM(Pin(13))

potentiometer = ADC(Pin(27))

reading = 0

while True:
  time.sleep(0.01)
  reading = potentiometer.read_u16()

  buzzer.freq(500)
  buzzer.duty_u16(reading)
  print(reading)