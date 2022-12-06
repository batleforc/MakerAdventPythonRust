# main.py -- put your code here!
from machine import ADC, Pin,PWM
import time

green = PWM(Pin(18))
amber = PWM(Pin(19))
red = PWM(Pin(20))

buzzer = PWM(Pin(13))

potentiometer = ADC(Pin(27))

buzzer.freq(1000)

green.freq(1000)
amber.freq(1000)
red.freq(1000)

buzzer.duty_u16(10000)
time.sleep(1)
buzzer.duty_u16(0)