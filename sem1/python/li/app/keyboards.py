from aiogram.types import (ReplyKeyboardMarkup, KeyboardButton, InlineKeyboardMarkup, InlineKeyboardButton)

main = ReplyKeyboardMarkup(keyboard=[[KeyboardButton(text='Соц.Сети')],[KeyboardButton(text='Помощь')],[KeyboardButton(text='Выйти')]], resize_keyboard=True, input_field_placeholder='Выберите вариант')

smedia = InlineKeyboardMarkup(inline_keyboard=[[InlineKeyboardButton(text='YouTube', callback_data='yt')],[InlineKeyboardButton(text='Twitch', callback_data='twitch')],[InlineKeyboardButton(text='Telegram Group', callback_data='tggroup')],[InlineKeyboardButton(text='VKontakte', callback_data='vk')],[InlineKeyboardButton(text='Steam', callback_data='steam')],[InlineKeyboardButton(text='Назад', callback_data='back')]], resize_keyboard=True, input_field_placeholder='Выберите соц.сеть')