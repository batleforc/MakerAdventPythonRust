# main.py -- put your code here!
from machine import Pin, PWM
import onewire, ds18x20, time
from time import sleep

red = Pin(20, Pin.OUT)
amber = Pin(19, Pin.OUT)
green = Pin(18, Pin.OUT)

SensorPin = Pin(26, Pin.IN)

sensor = ds18x20.DS18X20(onewire.OneWire(SensorPin))

buzzer = PWM(Pin(13))
buzzer.duty_u16(0)

red.value(0)
amber.value(0)
green.value(0)

roms = sensor.scan()
print('Found DS devices: ', roms)

while True:
  sensor.convert_temp()
  sleep(2)

  for rom in roms:
    print('Temperature: ', sensor.read_temp(rom), "°C")