from color_index import ColorIndex
import win32gui
import pyautogui
import time
import pydirectinput
import random

# 每次执行间隔时间
pydirectinput.PAUSE = 0.005

row_colors = ["#000000","#332211","#664422","#996633","#cc8844","#ffaa55","#32cc66","#65ee77","#981088","#cb3299","#fe54aa","#3176bb"]
col_colors = ["#000000","#332211","#664422","#996633","#cc8844","#ffaa55","#32cc66","#65ee77","#981088","#cb3299","#fe54aa","#3176bb","#6498cc","#97badd","#cadcee","#fdfeff"]
block_size = (80, 60)
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
    dicts = _color_dict()

    for cnt in range(count):
        # 选择拼图
        print("开始游戏，当前轮次：%d", cnt)
        click(image_position)
        time.sleep(0.1)
        # 点击确认按钮开始游戏
        click(start_position)
        time.sleep(0.2)
        is_success = True
        for _ in range(192):
            # 选取图块的上下部分颜色
            color1 = rgb_to_hex(pyautogui.pixel(choose_point_top[0], choose_point_top[1]))
            color2 = rgb_to_hex(pyautogui.pixel(choose_point_bottom[0], choose_point_bottom[1]))
            key = ColorIndex(color1, color2)
            # 获取坐标
            point = dicts.get(key)
            if point is not None:
                # 计算放置位置
                y = point[1] * block_size[0] + int(block_size[0] / 2) + offset[0]
                x = point[0] * block_size[1] + int(block_size[1] / 2) + offset[1]
                # 先移动到选取图块位置选取拼图
                click(choose_point_top)
                # 移动到对应位置放下拼图
                click((y, x))
            else:
                is_success = False
                break
        if is_success == True:
            print("当前轮次已结束，即将进行下一轮游戏...")
            # 结束一轮后暂停1.5s等待游戏动画结束
            time.sleep(2)
            # 鼠标点击开始下一轮
            click(confirm_position)
        else:
            print("数据解析失败，程序即将退出，请确认参数是否正确！")
            break

def rgb_to_hex(rgb: tuple) -> str:
    hex_color = '#{:02x}{:02x}{:02x}'.format(rgb[0], rgb[1], rgb[2])  
    return hex_color  

def click(point: tuple):
    _click(point[0], point[1])

def _click(dx, dy):
    pydirectinput.mouseDown(x=dx, y=dy)
    time.sleep(0.001)
    pydirectinput.mouseUp()

def test_move():
    # 定义随机x坐标和y坐标的范围  
    x_range = (0, 1920)  
    y_range = (0, 500) 
    cur = (1000, 900)

    times = 0.0
    for _ in range(192):
        start = time.time_ns()
        _click(cur[0], cur[1])
        # 生成随机x坐标和y坐标  
        random_x = random.randint(x_range[0], x_range[1])  
        random_y = random.randint(y_range[0], y_range[1])  
        _click(random_x, random_y)
        end = time.time_ns()
        times = times + (end - start)
    
    print(f'花费：{times / 1000000000}秒')

if __name__ == '__main__':
    # 聚焦应用程序
    hwnd = win32gui.FindWindow(None, "nu.exe")
    if hwnd:
        win32gui.SetForegroundWindow(hwnd)
        auto_puzzle()
    else:
        print("未找到窗口")
