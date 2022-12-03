while True:
  time.sleep(0.2)

  if button1.value() == 1:
    green.value(1)
  else :
    green.value(0)
  if button2.value() == 1:
    amber.value(1)
  else :
    amber.value(0)
  if button3.value() == 1:
    red.value(1)
  else :
    red.value(0)