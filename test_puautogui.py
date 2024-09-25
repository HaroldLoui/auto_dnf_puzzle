import pyautogui
import win32gui

def main():
    # 获取屏幕分辨率（宽高） Size(width=1920, height=1080)
    screenWidth, screenHeight = pyautogui.size()
    print(screenWidth, screenHeight)
    
    # 参数：横坐标、竖坐标，从屏幕左上角开始
    color1 = pyautogui.pixel(429, 1028)
    print(color1) 
    color2 = pyautogui.pixel(1250, 650)
    print(color2) 

    # 光标移动到(100, 200)位置, 0.02秒执行
    pyautogui.moveTo(140, 218)   
    pyautogui.click(button="right")  

    # 聚焦应用程序
    hwnd = win32gui.FindWindow(None, "nu.exe");
    if hwnd:
        win32gui.SetForegroundWindow(hwnd);
    else:
        print("未找到窗口");

if __name__ == "__main__":
    main()