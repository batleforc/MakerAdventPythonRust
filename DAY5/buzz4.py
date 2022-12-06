# main.py -- put your code here!
from machine import ADC, Pin,PWM
import time

buzzer = PWM(Pin(13))

C = 523
D = 587
E = 659
G = 784

volume = 32768

def playtone(note,vol,delay1,delay2):
    buzzer.duty_u16(vol)
    buzzer.freq(note)
    time.sleep(delay1)
    buzzer.duty_u16(0)
    time.sleep(delay2)

playtone(E,volume,0.1,0.2)
playtone(E,volume,0.1,0.2)
playtone(E,volume,0.1,0.5) #Longer second delay

playtone(E,volume,0.1,0.2)
playtone(E,volume,0.1,0.2)
playtone(E,volume,0.1,0.5) #Longer second delay

playtone(E,volume,0.1,0.2)
playtone(G,volume,0.1,0.2)
playtone(C,volume,0.1,0.2)
playtone(D,volume,0.1,0.2)
playtone(E,volume,0.1,0.2)

# Duty to 0 to turn the buzzer off
buzzer.duty_u16(0)