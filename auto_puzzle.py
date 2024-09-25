from color_index import ColorIndex
from pynput.mouse import Button, Controller
import win32gui
import pyautogui
import time

row_colors = ["#000000","#332211","#664422","#996633","#cc8844","#ffaa55","#32cc66","#65ee77","#981088","#cb3299","#fe54aa","#3176bb"]
col_colors = ["#000000","#332211","#664422","#996633","#cc8844","#ffaa55","#32cc66","#65ee77","#981088","#cb3299","#fe54aa","#3176bb","#6498cc","#97badd","#cadcee","#fdfeff"]
block_size = [80, 60]
offset = (55, 130)
choose_point_top = (170, 970)
choose_point_bottom = (170, 1000)
image_position = (500, 250)
start_position = (700, 1050)
confirm_position = (700, 1100)
count = 50

def _color_dict() -> dict:
    color_dicts = {}
    for i in range(len(row_colors)):
        for j in range(len(col_colors)):
            key = ColorIndex(col_colors[j], row_colors[i])
            color_dicts[key] = (i, j)
    
    return color_dicts

def auto_puzzle():
    dicts = _color_dict();

    mouse = Controller()
    for cnt in range(count):
        # 选择拼图
        print("开始游戏，当前轮次：%d", cnt)
        mouse.position = image_position
        mouse.click(Button.left)
        # 点击确认按钮开始游戏
        mouse.position = start_position
        mouse.click(Button.left)
        time.sleep(0.2)
        is_success = True
        for _ in range(192):
            # 选取图块的上下部分颜色
            color1 = pyautogui.pixel(choose_point_top[0], choose_point_top[1])
            color2 = pyautogui.pixel(choose_point_bottom[0], choose_point_bottom[1])
            key = ColorIndex(color1, color2)
            # 获取坐标
            point = dicts.get(key)
            if point is not None:
                # 计算放置位置
                y = int(point[1] * block_size[0] + block_size[0] / 2 + offset[0])
                x = int(point[0] * block_size[1] + block_size[1] / 2 + offset[1])
                # 先移动到选取图块位置
                mouse.position = choose_point_top
                mouse.click(Button.left)
                # 移动到对应位置
                mouse.position = (y, x)
                mouse.click(Button.left)
            else:
                is_success = False
                break
        if is_success == True:
            print("当前轮次已结束，即将进行下一轮游戏...");
            # 结束一轮后暂停1.5s等待游戏动画结束
            time.sleep(2);
            # 鼠标点击开始下一轮
            mouse.position = confirm_position
            mouse.click(Button.left)
        else:
            print("数据解析失败，程序即将退出，请确认参数是否正确！");
            break

def start():
    # 聚焦应用程序
    hwnd = win32gui.FindWindow(None, "nu.exe");
    if hwnd:
        win32gui.SetForegroundWindow(hwnd);
        auto_puzzle()
    else:
        print("未找到窗口");

if __name__ == '__main__':
    mouse = Controller()
    mouse.position = start_position

    dicts = _color_dict()
    
    for key, value in dicts.items():
        print(f'{str(key)} ==> {value}')
