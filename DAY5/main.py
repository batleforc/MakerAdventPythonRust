# main.py -- put your code here!
from machine import ADC, Pin,PWM
import time

buzzer = PWM(Pin(13))

C = 523
D = 587
E = 659
G = 784

volume = 10000

# Play the tune

# "Jin..."
buzzer.duty_u16(volume) # Volume up
buzzer.freq(E) # Set frequency to the E note
time.sleep(0.1) # Delay
buzzer.duty_u16(0) # Volume off
time.sleep(0.2) # Delay

# "...gle"
buzzer.duty_u16(volume)
buzzer.freq(E)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.2)

# "Bells"
buzzer.duty_u16(volume)
buzzer.freq(E)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.5) # longer delay

# "Jin..."
buzzer.duty_u16(volume)
buzzer.freq(E)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.2)

# "...gle"
buzzer.duty_u16(volume)
buzzer.freq(E)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.2)

# "Bells"
buzzer.duty_u16(volume)
buzzer.freq(E)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.5) # longer delay

# "Jin..."
buzzer.duty_u16(volume)
buzzer.freq(E)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.2)

# "...gle"
buzzer.duty_u16(volume)
buzzer.freq(G)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.2)

# "All"
buzzer.duty_u16(volume)
buzzer.freq(C)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.2)

# "The"
buzzer.duty_u16(volume)
buzzer.freq(D)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.2)

# "Way"
buzzer.duty_u16(volume)
buzzer.freq(E)
time.sleep(0.1)
buzzer.duty_u16(0)
time.sleep(0.2)

# Duty to 0 to turn the buzzer off
buzzer.duty_u16(0)