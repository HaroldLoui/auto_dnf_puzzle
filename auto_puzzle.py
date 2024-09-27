from color_index import ColorIndex
import win32gui
import pyautogui
import time
import pydirectinput

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
        print("开始游戏，当前轮次：%d" % cnt)
        click(image_position[0], image_position[1])
        time.sleep(0.05)
        # 点击确认按钮开始游戏
        click(start_position[0], start_position[1])
        time.sleep(0.2)
        is_success = True
        for _ in range(192):
            point = None
            num = 0
            while point is None:
                num += 1
                # 选取图块的上下部分颜色
                color1 = rgb_to_hex(pyautogui.pixel(choose_point_top[0], choose_point_top[1]))
                color2 = rgb_to_hex(pyautogui.pixel(choose_point_bottom[0], choose_point_bottom[1]))
                key = ColorIndex(color1, color2)
                # 获取坐标
                point = dicts.get(key)
                if point is not None or num >= 20:
                    break
                else:
                    time.sleep(0.05)

            if point is not None:
                # 计算放置位置
                x = point[1] * block_size[0] + int(block_size[0] / 2) + offset[0]
                y = point[0] * block_size[1] + int(block_size[1] / 2) + offset[1]
                # 先移动到选取图块位置选取拼图
                while click(choose_point_top[0], choose_point_top[1]) == False:
                    pass
                # 移动到对应位置放下拼图
                click(x, y)
            else:
                is_success = False
                break
        if is_success == True:
            print("当前轮次已结束，即将进行下一轮游戏...")
            # 结束一轮后暂停1.5s等待游戏动画结束
            time.sleep(2)
            # 鼠标点击开始下一轮
            click(confirm_position[0], confirm_position[1])
        else:
            print("数据解析失败，程序即将退出，请确认参数是否正确！")
            break

def rgb_to_hex(rgb: tuple) -> str:
    hex_color = '#{:02x}{:02x}{:02x}'.format(rgb[0], rgb[1], rgb[2])  
    return hex_color  

def click(dx, dy) -> bool:
    try:
        pydirectinput.mouseDown(x=dx, y=dy)
        time.sleep(0.005)
        pydirectinput.mouseUp()
        time.sleep(0.02)
        return True
    except:
        return False

if __name__ == '__main__':
    # 聚焦应用程序
    hwnd = win32gui.FindWindow(None, "nu.exe")
    if hwnd:
        win32gui.SetForegroundWindow(hwnd)
        auto_puzzle()
    else:
        print("未找到窗口")
