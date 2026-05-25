from typing import Optional

from sqlmodel import Field, SQLModel


class Task(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)
    title: str
    description: str = ""
    is_done: bool = False

    todo_list_id: Optional[int] = Field(default=None, foreign_key="todolist.id")