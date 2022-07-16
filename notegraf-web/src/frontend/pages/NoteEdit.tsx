import {useParams} from "react-router-dom";
import * as React from "react";
import {useEffect, useState} from "react";
import {getNote} from "../api";
import {NoteForm} from "../components/NoteForm";
import {tileInTitle} from "../utils";

export function NoteEdit() {
    let {noteID} = useParams();
    const [note, setNote] = useState<any>(null);
    const [error, setError] = useState<any>(null);
    const [isLoaded, setIsLoaded] = useState(false);

    useEffect(() => {
        async function fetchNote() {
            try {
                const note = await getNote(noteID as string);
                setNote(note);
                setIsLoaded(true);
            } catch (e) {
                setError(e);
                setIsLoaded(true);
            }
        }

        fetchNote();
    }, [noteID]);


    if (!isLoaded) {
        return (<div>Loading...</div>);
    }
    if (error) {
        return (<div>{error.toString()}</div>);
    }

    return (<NoteForm
        defaultValue={{
            title: note.title,
            note_inner: note.note_inner,
            metadata_tags: note.metadata.tags.join(", "),
            metadata_custom_metadata: JSON.stringify(note.metadata.custom_metadata)
        }}
        endpoint={`note/${note.id}/revision`}
        autoSaveKey={`autosave.note.edit.${note.id}`}
        submitText={"Update"}
        title={`Update note ${tileInTitle(note.title)} - Notegraf`}
    />);
}