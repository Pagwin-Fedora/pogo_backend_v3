use crate::serial_task::TaskSerial;
use crate::postgres_connection as pg_conn;
use uuid::Uuid;

type Res = Result<(),crate::error_handling::Error>;

/// Mom can get get ORM?
/// No we have ORM at home
/// The ORM at home
pub async fn update_task(TaskSerial{title,body,progress,children,parents,media}:TaskSerial, id:Uuid)->Res{
    // as repetitive as this is the alternative is writing a macro so yeah, no
    // also I hate the fact I'm not doing everything in 1 query just as much as you future me
    let mut futs:Vec<Box<dyn std::future::Future<Output = Res>>>> = Vec::with_capacity(8);
    if let Some(title) = title {
        futs.push(update_title(title,&id).into());
    }
    if let Some(body) = body{
        futs.push(update_body(body,&id).into());
    }
    if let Some(progress) = progress{
        futs.push(update_progress(progress,&id).into());
    }
    if let Some(children) = children{
        futs.push(update_children(children,&id).into());
    }
    if let Some(parents) = parents{
        futs.push(update_parents(parents,&id).into());
    }
    if let Some(media) = media{
        futs.push(update_media(media,&id).into());
    }
    //there's no reason to parallelize this or otherwise make it concurrent but screw it I'm bored
    let mut tmp = Vec::with_capacity(futs.len());
    for fut in futs {
        tmp.push(tokio::spawn(fut));
    }
    for fut in tmp{
        fut.await?;
    }
    Ok(())
}
async fn update_title(title:String, id:&Uuid)->Res{
    sqlx::query!("UPDATE pogo_tasks SET title = $1 WHERE id=$2",title,id)
        .execute(&pg_conn::get_handle()).await?;
    Ok(())
}
async fn update_body(body:String, id:&Uuid)->Res{
    todo!("update body");
}
async fn update_progress(prog:f32, id:&Uuid)->Res{
    todo!("update progress");
}
async fn update_children(children:Vec<Uuid>,id:&Uuid)->Res{
    todo!("Update children");
}
async fn update_parents(parents:Vec<Uuid>,id:&Uuid)->Res{
    todo!("Update parents");
}
async fn update_media(media: Vec<Uuid>,id: &Uuid)->Res{
    todo!("Update Media");
}
