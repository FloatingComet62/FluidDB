const express = require('express');
const app = express();
const port = 3000;
const database = require('../database/high');

app.listen(port, ()=> console.log(`API running on http://127.0.0.1:${port}`));

app.get('/', (_, response)=>{
    response.status(200).send({
        data: database.getAll(),
        error: null
    });
});

app.get('/reset', (_, response)=>{
    database.reset();
    response.status(200).send({
        data: 'Done',
        error: null
    })
});

app.use('/ocean/:name', (request, response)=>{
    const name = request.params.name;
    if(request.method === 'GET'){
        const data = database.getOcean(name);
        if(data){
            response.status(200).send({
                data: data,
                error: null,
            });
        }else{
            response.status(400).send({
                data: null,
                error: `No ocean with name ${name}`,
            });
        }
    }else if(request.method === 'POST'){
        database.addOcean(name);
        response.status(200).send({
            data: database.getOcean(name),
            error: null,
        });
    }
});

app.use('/sea/:oceanname/:name', (request, response)=>{
    const name = request.params.name;
    const oceanname = request.params.oceanname;
    if(request.method === 'GET'){
        const data = database.getSea(oceanname, name);
        if(data){
            response.status(200).send({
                data: data,
                error: null,
            });
        }else{
            response.status(400).send({
                data: null,
                error: `No ocean with name ${name}`,
            });
        }
    }else if(request.method === 'POST'){
        database.addSea(oceanname, name);
        response.status(200).send({
            data: database.getSea(oceanname, name),
            error: null,
        });
    }
});

app.use('/river/:oceanname/:seaname/:name', (request, response)=>{
    const name = request.params.name;
    const oceanname = request.params.oceanname;
    const seaname = request.params.seaname;
    if(request.method === 'GET'){
        const data = database.getRiver(oceanname, seaname, name);
        if(data){
            response.status(200).send({
                data: data,
                error: null,
            });
        }else{
            response.status(400).send({
                data: null,
                error: `No ocean with name ${name}`,
            });
        }
    }else if(request.method === 'POST'){
        database.addRiver(oceanname, seaname, name);
        response.status(200).send({
            data: database.getRiver(oceanname, seaname, name),
            error: null,
        });
    }
});