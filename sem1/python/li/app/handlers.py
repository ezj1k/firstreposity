from aiogram import F, Router
from aiogram.types import Message, CallbackQuery, ReplyKeyboardRemove
from aiogram.filters import CommandStart, Command

import app.keyboards as kb

router=Router()

@router.message(CommandStart())
async def cmd_start(message: Message):
    await message.reply('Здравствуйте, вы запустили бота!', reply_markup=kb.main)

@router.message(Command('help'))
@router.message(F.text == 'Помощь')
async def cmd_help(message: Message):
    help_text = (
        'Это бот, созданный для практического упражнения по предмету Python. '
        'Он позволяет более легкий и удобный серфинг между социальными сетями Юлия.\n\n'
        'Выбрав вариант "Соц.Сети", вы можете выбрать любую из доступных соцсетей, '
        'нажав на нее, после чего бот выдаст ссылку на нее.\n\n'
        'Кнопка "Назад" в меню соцсетей вернет вас в главное меню.\n'
        'Кнопка "Выйти" завершит сеанс работы с ботом (уберет клавиатуру).'
    )
    await message.reply(help_text, reply_markup=kb.main)


# @router.message(F.text == 'Помощь')
# async def cmd_help(message: Message):
#     await message.reply('Это бот созданный для практичного упражнения по предмету Python. Он позволяет более легкий и удобный серфинг между социальныи сетями Юлия. Фактически ты можешь выбрать любую из доступных здесь соц сетей, нажав на нее, после чего тебе выдаст ссылку на нее. Если написать \'Назад\' то вас вернет назад, в главное меню.\n')

@router.message(F.text == 'Соц.Сети')
async def smedia(message: Message):
    await message.reply('Выберите соц.сеть чтоб узнать ссылку', reply_markup=kb.smedia)
    await message.answer(reply_markup=ReplyKeyboardRemove())

@router.callback_query(F.data == 'yt')
async def yt(callback: CallbackQuery):
    await callback.answer('Вы выбрали YouTube!')
    await callback.message.reply('Вот тебе ссылка на ютуб, не забывай подписываться!\nhttps://www.youtube.com/@ezj1k\n')

@router.callback_query(F.data == 'twitch')
async def twitch(callback: CallbackQuery):
    await callback.answer('Вы выбрали Twitch!')
    await callback.message.reply('Вот тебе ссылка на твич, не забывай фолловиться!\nhttps://www.twitch.tv/ezj1kk?sr=a\n')

@router.callback_query(F.data == 'tggroup')
async def tggroup(callback: CallbackQuery):
    await callback.answer('Вы выбрали группу теллеграм!')
    await callback.message.reply('Вот тебе ссылка на группу в теллеграмме, подписывайся если интерестно!\nhttps://t.me/ezj1kkk\n')

@router.callback_query(F.data == 'vk')
async def vk(callback: CallbackQuery):
    await callback.answer('Вы выбрали вконтакте!')
    await callback.message.reply('Вот тебе ссылка на мою страничку в вк, я там редко сижу поэтому можешь и не подписываться!\nhttps://vk.com/lupascu_iulii\n')

@router.callback_query(F.data == 'steam')
async def steam(callback: CallbackQuery):
    await callback.answer('Вы выбрали стим!')
    await callback.message.reply('Вот тебе ссылка на аккаунт стим, не знаю зачем он мог тебе понадобиться!\nhttps://steamcommunity.com/id/ezj1k/\n')

@router.callback_query(F.data == 'back')
async def back(callback: CallbackQuery):
    await callback.answer('Возвращаемся в главное меню...')
    await callback.message.edit_text('Вы вернулись в главное меню.\nИспользуйте кнопки ниже для навигации.', reply_markup=None)
    await callback.message.answer('Используйте кнопки ниже для навигации:', reply_markup=kb.main)

@router.message(F.text == 'Выйти')
async def cmd_exit(message: Message):
    await message.reply('До свидания! Чтобы снова пообщаться, используйте команду /start.', reply_markup=ReplyKeyboardRemove())