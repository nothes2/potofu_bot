import os
import discord 
from discord.ext import commands
from dotenv import load_dotenv

load_dotenv()
ENV = os.getenv('ENV')

intents = discord.Intents.default()
intents.message_content = True

bot = commands.Bot(command_prefix="!", intents=intents)

@bot.event
async def on_ready():
    print(f'Logged in as {bot.user}')

@bot.event
async def on_message(message):
    if message.author == bot.user:
        return
    
    if message.content.startswith('hello'):
        await message.channel.send(f'Hello, {message.author}!')
        
    await bot.process_commands(message)

@bot.command()
async def greet(ctx, *, name: str):
    await ctx.send(f'Hello, {name}!')

try:
    bot.run(ENV)
except Exception as e:
    print(e)