#kivy
from kivy.app import App
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.gridlayout import GridLayout
from kivy.lang import Builder
from kivy.uix.togglebutton import ToggleButton
from kivy.uix.button import Button
from kivy.properties import NumericProperty
from kivy.core.window import Window
from kivy.clock import Clock

#spark
import requests

accessToken = "6d6873caba909d7f00dde47440fe7933e65582e6"
deviceID = "23001e000947353138383138"


activeColor = (1,1,1, 1)
disabledColor = (0.5,0.5,0.5,0.5)

Builder.load_string("""
<CasoDisplay>:
    #size_hint: None
    size: (200, 400)
    OnOffButton:
        #size_hint: (None, None)
        id: onOff
        text: 'on/off'
        on_press: root.on_off()
    PowerModeButton:
        #size_hint: (None, None)
        id: powerMode
        text: 'Power'
        on_press: root.power_mode()
        disabled: 'True'
    BoxLayout:
        #size_hint: (None, None)
        orientation: 'vertical'
        PowerUpDownButton:
            #size_hint: (None, None)
            id: up
            text: 'up'
            disabled: 'True'
            on_press: root.power_adjust('up')
        PowerUpDownButton:
            #size_hint: (None, None)
            id: down
            text: 'down'
            disabled: 'True'
            on_press: root.power_adjust('down')
    Label:
        #size_hint: (None, None)
        id: powerLED
        text: str(root.powerLevel)
        disabled: 'True'
        color: (0,1,0,1)
        font_size: 72
        font_name: 'SFDigitalReadout-Medium' #/Users/frodebergolsen/Library/Fonts/SFDigitalReadout-Medium.ttf
    Label:
        #size_hint: (None, None)
        id: temp
        text: str(root.temp)
        disabled: 'True'
        color: (0,1,0,1)
        font_size: 72
        font_name: 'SFDigitalReadout-Medium' #/Users/Frode/Library/Fonts/SFDigitalReadout-Medium.ttf

""")

class Particle():

    def __init__(self, access_token, device_id):
        self.__access_token = access_token
        self.__device_id = device_id
        self.__temp = 0
        self.__isconnected = True

    def isconnected(self):
        return self.__isconnected

    def temp(self):
        self.__temp = self.__readTemp()
        return self.__temp

    def __readTemp(self):
        baseUrl = "https://api.spark.io/v1/devices"
        url = baseUrl + '/' + self.__device_id  + '/tempc'
        payLoad = {"access_token": self.__access_token, "format": "raw"}
        return requests.get(url, params=payLoad).json()


class CasoController():

    __url = "https://api.spark.io/v1/devices/" + deviceID + "/relay"
    __onPayload = {"access_token": accessToken, "params": 'r1,HIGH'}
    __offPayload = {"access_token": accessToken, "params": 'r1,LOW'}

    def blink(self, relay):
        self.__onPayload['params'] = relay + ',HIGH'
        self.__offPayload['params'] = relay + ',LOW'
        r = requests.post(self.__url, data=self.__onPayload)
        r = requests.post(self.__url, data=self.__offPayload)

    def power_on(self):
        print('Power ON!')

        self.__onPayload['params'] = 'r1,HIGH'
        self.__offPayload['params'] = 'r1,LOW'
        r = requests.post(self.__url, data=self.__onPayload)
        r = requests.post(self.__url, data=self.__offPayload)

        #print(r.json().get('result'))

    def power_off(self):
        print('Power OFF!')
        self.__onPayload['params'] = 'r1,HIGH'
        self.__offPayload['params'] = 'r1,LOW'
        r = requests.post(self.__url, data=self.__onPayload)
        r = requests.post(self.__url, data=self.__offPayload)


    def power_mode_on(self):
        print('Power Mode On!!')

        self.__onPayload['params'] = 'r2,HIGH'
        self.__offPayload['params'] = 'r2,LOW'

        r = requests.post(self.__url, data=self.__onPayload)
        r = requests.post(self.__url, data=self.__offPayload)

    def power_adjust(self, up_down):
        if up_down == 'up':
            print('up')
            self.blink('r3')
        elif up_down == 'down':
            self.blink('r4')
            print('down')
        else:
            print('CasoVontroller: Something is wrong!')


class OnOffButton(ToggleButton):
    pass

class PowerModeButton(ToggleButton):
    pass

class PowerUpDownButton(Button):
    pass


class CasoDisplay(BoxLayout):

    powerLevel = NumericProperty(0)
    temp = NumericProperty(0)
    p = Particle(accessToken, deviceID)

    def on_off(self):
        if self.ids['onOff'].state == 'normal': #off
            self.ids['powerMode'].disabled = True
            self.ids['powerMode'].state = 'normal'
            self.ids['up'].disabled = True
            self.ids['down'].disabled = True
            self.ids['powerLED'].disabled = True
            self.ids['powerLED'].text = ''
            Caso.power_off()
        else:                                   #on
            self.ids['powerMode'].disabled = False
            self.ids['powerLED'].disabled = False
            self.ids['powerLED'].text = ''
            Caso.power_on()

    def power_mode(self):
        if self.ids['powerMode'].state == 'down':
            self.ids['up'].disabled = False
            self.ids['down'].disabled = False
            self.ids['powerLED'].disabled = False
            self.powerLevel = 5
            Caso.power_mode_on()
        else:
            pass
            #self.ids['powerMode'].state = 'down'

    def power_adjust(self, button):
        if button == 'up':
            if self.powerLevel < 9:
                self.powerLevel+=1
                Caso.power_adjust('up')
        elif button == 'down':
            if self.powerLevel > 1:
                self.powerLevel-=1
                Caso.power_adjust('down')
        else:
            print('Something is wrong!')

    def update_temp(self, dt):
        #print(p.temp())
        self.temp = p.temp()


class MainApp(App):
    def build(self):
        cs = CasoDisplay()
        Clock.schedule_interval(cs.update_temp, 1)
        return cs

p = Particle(accessToken, deviceID)

Caso = CasoController()
Window.size = (600, 200)


if __name__ == '__main__':
    MainApp().run()
