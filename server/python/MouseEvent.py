
import mouse as m

LONG_TAP_TIMEOUT = 500
DRAG_TIMEOUT = 500
DEBUG = False

class MouseEvent:
    state = None
    paction = None
    action = None
    dx = 0.0
    px = 0.0
    x = 0.0
    dy = 0.0
    py = 0.0
    y = 0.0
    eventTime = 0
    downTime = 0
    mul = 3
    def act(self, message):
        if "MotionEvent" in message:
            fields = message\
                .replace("MotionEvent","")\
                .replace(" ","")\
                .strip("{} ")\
                .split(",")
            for field in fields:
                if "action" in field:
                    self.paction = self.action
                    self.action = field.split("=")[1]
                if "x[0]" in field:
                    self.px = self.x
                    self.x = float(field.split('=')[1])
                if "y[0]" in field:
                    self.py = self.y
                    self.y = float(field.split('=')[1])
                if "eventTime" in field:
                    self.eventTime = float(field.split('=')[1])
                if "downTime" in field:
                    self.downTime = float(field.split('=')[1])
        self.__do()
    
    def __do(self):
        self.dx = self.x - self.px
        self.dy = self.y - self.py 
        
        if self.paction == "ACTION_DOWN":
            if self.action == "ACTION_UP":
                if m.is_pressed():
                    debug("releasing")
                    m.release()
                    return

                if self.eventTime - self.downTime < LONG_TAP_TIMEOUT:
                    debug("L Click")
                    m.click()
                else:
                    debug("R Click")
                    m.click("right")
                return

            if self.action == "ACTION_MOVE":
                if self.eventTime - self.downTime > DRAG_TIMEOUT:
                    if not m.is_pressed():
                        debug("pressing")
                        m.press()
                m.move(self.dx, self.dy, absolute=False)
                return

        if self.paction == "ACTION_MOVE":
                if self.action == "ACTION_MOVE":
                    m.move( self.mul * self.dx, self.mul * self.dy, absolute=False)
                    return
                if self.action == "ACTION_UP":
                    self.dx = 0
                    self.dy = 0

def debug(message):
    if DEBUG:
        print(message)