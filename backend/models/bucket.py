from typing import List, Optional

from sqlmodel import Field, Relationship, SQLModel

from .todo_list import TodoList


class Bucket(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)
    name: str

    todo_lists: List[TodoList] = Relationship(back_populates="bucket")