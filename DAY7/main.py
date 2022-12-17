# main.py -- put your code here!
from machine import Pin, PWM
from time import sleep

red = Pin(20, Pin.OUT)
amber = Pin(19, Pin.OUT)
green = Pin(18, Pin.OUT)

pir  = Pin(26, Pin.IN, Pin.PULL_DOWN)

buzzer = PWM(Pin(13))
buzzer.duty_u16(0)

red.value(1)
amber.value(0)
green.value(0)

sleep(10)

red.value(0)
amber.value(1)

def alarm():
  buzzer.duty_u16(10000)

  for i in range(5):
    buzzer.freq(5000)
    green.value(1)
    amber.value(0)
    red.value(0)
    sleep(1)
    red.value(1)
    green.value(0)
    buzzer.freq(500)
    sleep(1)
  buzzer.duty_u16(0)
  amber.value(1)
  green.value(0)
  red.value(0)

while True:
  sleep(0.01)
  if pir.value() == 1:
    alarm()