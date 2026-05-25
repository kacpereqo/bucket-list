from typing import List, Optional

from sqlmodel import Field, Relationship, SQLModel

from .task import Task


class TodoList(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)
    name: str

    bucket_id: Optional[int] = Field(default=None, foreign_key="bucket.id")

    tasks: List[Task] = Relationship(back_populates="todo_list")