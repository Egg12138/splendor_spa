import logging
from config import *

def setup_logger(name, log_file, level=logging.INFO):

    # formatter = logging.Formatter('%(asctime)s %(levelname)s [%(lineno)d]')
    formatter = logging.Formatter('[%(lineno)d]$%(message)s')

    handler = logging.FileHandler(log_file)        
    handler.setFormatter(formatter)

    logger = logging.getLogger(name)
    logger.setLevel(level)
    if not logger.handlers:
        logger.addHandler(handler)

    return logger


def actcode_to_action(code):
    print(f"{code}-->...")
    if code < 90:
        action = _card_content(code)    
        return action
    elif code < 95:
        gems_coloridx = code - 90
        action = "Picked " + f"{GEMS_ICONS[gems_coloridx]}"*2
        return action
    else:
        combination_id = code - 95
        action = f"Picked {diff_color_map[combination_id]}"
        return action


def _card_content(code):
    """
    return a card_content string, like:
    [🔲][0]0🟢 0⚪ 0💎 0🟤 3🔴
    """
    card_vec = cards_pool[code]
    card_color = CARD_COLORS[card_vec[COLOR_I]-1]
    card_score = card_vec[SCORE_I]
    content = f"Bought [{card_color}][{card_score}]"
    for i in range(5):
        cost = card_vec[i]
        color = GEMS_ICONS[i] 
        content += f"{cost}{color} "
    return content

if __name__ == '__main__':
    for i in range(ACTIONS_NUM):
        print(actcode_to_action(i))