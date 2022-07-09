import {Note} from "./note";

type NoteLocator = {
    Specific: string[]
}

export async function getNote(noteID: string): Promise<Note> {
    const response = await fetch(`/api/v1/note/${noteID}`);
    if (!response.ok) {
        throw new Error(response.statusText);
    }
    return response.json();
}

export async function deleteNote(noteID: string) {
    const response = await fetch(`/api/v1/note/${noteID}`, {
        method: "DELETE"
    });
    if (!response.ok) {
        throw new Error(response.statusText);
    }
}

export async function postNote(data: any): Promise<NoteLocator> {
    let response = await fetch('/api/v1/note', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    });
    if (!response.ok) {
        console.error('Error:', response.statusText);
    }
    return response.json();
}

export async function updateNote(noteID: string, data: any): Promise<NoteLocator> {
    let response = await fetch(`/api/v1/note/${noteID}/revision`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    });
    if (!response.ok) {
        console.error('Error:', response.statusText);
    }
    return response.json();
}