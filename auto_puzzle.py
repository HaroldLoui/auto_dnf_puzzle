from color_index import ColorIndex
import pydirectinput
import win32gui
import pyautogui
import time

# 每次事件操作后暂停的时间（秒），默认0.1
# 注释或者改大程序会运行更加稳定，但耗费时间会更长（我自己的电脑是13秒一把）
# 出现不稳定的情况，就放弃当前游戏，重新运行脚本即可
pydirectinput.PAUSE = 0.005

row_colors = ["#000000","#332211","#664422","#996633","#cc8844","#ffaa55","#32cc66","#65ee77","#981088","#cb3299","#fe54aa","#3176bb"]
col_colors = ["#000000","#332211","#664422","#996633","#cc8844","#ffaa55","#32cc66","#65ee77","#981088","#cb3299","#fe54aa","#3176bb","#6498cc","#97badd","#cadcee","#fdfeff"]

# ================== 需要修改的参数start ==================
block_size = [80, 60]                   # 单个拼图图块的大小
offset = (55, 100)                      # 拼图区域离左上角的偏移量
choose_point_top = (170, 970)           # 待选择拼图图块上部分中心的位置
choose_point_bottom = (170, 1000)       # 待选择拼图图块下部分中心的位置
image_position = (500, 250)             # 打了补丁后第二张192块拼图中心的位置
start_position = (700, 1050)            # 开始游戏按钮中心的位置
confirm_position = (700, 1100)          # 完成一轮游戏后点击确认按钮中心的位置
count = 50                              # 执行轮次，一轮50硬币
# ================== 需要修改的参数 end ==================

def _color_dict() -> dict:
    color_dicts = {}
    for i in range(len(row_colors)):
        for j in range(len(col_colors)):
            key = ColorIndex(col_colors[j], row_colors[i])
            color_dicts[key] = (i, j)
    
    return color_dicts

def auto_puzzle():
    dicts = _color_dict();

    total_time = 0.0
    cnt = 1
    while cnt <= count:
        # 选择拼图
        print(f"开始游戏，当前轮次：{cnt}")
        click(image_position[0], image_position[1])
        time.sleep(0.2)
        # 点击确认按钮开始游戏
        click(start_position[0], start_position[1])
        time.sleep(0.2)
        is_success = True
        game_loop = 0
        start = time.time_ns()
        while game_loop < 192:
            point = None
            num = 0
            # 选取图块的上下部分颜色
            while point is None:
                num = num + 1
                color1 = rgb_to_hex(pyautogui.pixel(choose_point_top[0], choose_point_top[1]))
                color2 = rgb_to_hex(pyautogui.pixel(choose_point_bottom[0], choose_point_bottom[1]))
                key = ColorIndex(color1, color2)
                # 获取坐标
                point = dicts.get(key)
                if point is not None or num > 30:
                    break
                else:
                    time.sleep(0.05)

            if point is not None:
                # 先移动到选取图块位置
                while click(choose_point_top[0], choose_point_top[1]) == False:
                    pass
                # time.sleep(0.03)
                # 计算放置位置
                x = block_size[0] * point[1] + offset[0] + int(block_size[0] / 2)
                y = block_size[1] * point[0] + offset[1] + int(block_size[1] / 2)
                # 移动到对应位置
                click(x, y)
                game_loop = game_loop + 1
            else:
                is_success = False
                break

        end = time.time_ns()
        times = end - start
        total_time += times
        if is_success == True and game_loop >= 192:
            cnt += 1
            print(f"当前轮次已结束（耗时：{times / 1_000_000_000}s），即将进行下一轮游戏...")
            # 结束一轮后暂停1.5s等待游戏动画结束
            time.sleep(2)
            # 鼠标点击开始下一轮
            click(confirm_position[0], confirm_position[1])
        else:
            print("数据解析失败，程序即将退出，请确认参数是否正确！")
            break

    total_seconds = total_time / 1_000_000_000
    print(f"所有轮次已结束，总耗时：{total_seconds}s，平均耗时：{total_seconds / cnt}s")

def rgb_to_hex(rgb):  
    # 格式化每个颜色值为两位的16进制数  
    hex_color = '#{:02x}{:02x}{:02x}'.format(rgb[0], rgb[1], rgb[2])  
    
    return hex_color 

def click(dx, dy) -> bool: 
    try:
        pydirectinput.mouseDown(x=dx, y=dy)
        time.sleep(0.005)
        pydirectinput.mouseUp()
        time.sleep(0.005)
        return True
    except:
        return False

if __name__ == '__main__':
    # 聚焦应用程序
    hwnd = win32gui.FindWindow(None, "地下城与勇士：创新世纪")
    if hwnd:
        win32gui.SetForegroundWindow(hwnd)
        auto_puzzle()
    else:
        print("未找到窗口")
