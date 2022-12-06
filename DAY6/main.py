# main.py -- put your code here!
from machine import ADC, Pin
from time import sleep

red = Pin(20, Pin.OUT)
amber = Pin(19, Pin.OUT)
green = Pin(18, Pin.OUT)

lightsensor = ADC(Pin(26))

def parse_light_sensor(lightsensor):
    light = lightsensor.read_u16()
    lightpercent = round(light/65535*100,1)
    return lightpercent

def pretty_light_sensor(lightsensor):
    lightpercent = parse_light_sensor(lightsensor)
    print("{}%".format(lightpercent))

while True:
  light = parse_light_sensor(lightsensor)

  sleep(1)
  if light <= 30:
    red.value(1)
    amber.value(0)
    green.value(0)
  elif light > 30 and light < 60:
    red.value(0)
    amber.value(1)
    green.value(0)
  elif light >= 60:
    red.value(0)
    amber.value(0)
    green.value(1)