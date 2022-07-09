import express, { Request, Response } from 'express'
import high from './database/high'
import { config } from 'dotenv'
import fs from 'fs'

const app = express()
config()

app.listen(process.env.PORT || '8080', () => {
    console.log(`Server is running on port ${process.env.PORT}`)
})
function logger(req: Request, res: Response, message: any) {
    const log = `${req.method} ${req.url} : ${res.statusCode} ${JSON.stringify(message)}`
    fs.appendFileSync('log.txt', log + '\n')
}

function endHandler(success: any, message: any, req: Request, res: Response, code: number, systemCode: number) {
    let outCode = 200
    if (systemCode===1) outCode = 400
    if (systemCode===2) outCode = 500

    if (!success) res.status(outCode)
    else res.status(code)

    res.json({ success, message, systemCode })
    logger(req, res, { success, message, systemCode })
}

app.get('/',(req, res) => {
    const { success, message, code } = high.getEveryThing()
    endHandler(success, message, req, res, 200, code)
})

app.get('/ocean/:oceanName', (req, res) => {
    const { success, message, code } = high.getOcean(req.params.oceanName)
    endHandler(success, message, req, res, 200, code)
})
app.post('/ocean/:oceanName', (req, res) => {
    const { success, message, code } = high.createOcean(req.params.oceanName)
    endHandler(success, message, req, res, 201, code)
})

app.get('/sea/:oceanName/:seaName', (req, res) => {
    const { success, message, code } = high.getSea(
        req.params.oceanName,
        req.params.seaName
    )
    endHandler(success, message, req, res, 200, code)
})
app.post('/sea/:oceanName/:seaName', (req, res) => {
    const { success, message, code } = high.createSea(
        req.params.oceanName,
        req.params.seaName
    )
    endHandler(success, message, req, res, 201, code)
})

app.get('/river/:oceanName/:seaName/:riverName', (req, res) => {
    const { success, message, code } = high.getRiver(
        req.params.oceanName,
        req.params.seaName,
        req.params.riverName
    )
    endHandler(success, message, req, res, 200, code)
})
app.post('/river/:oceanName/:seaName/:riverName/:value', (req, res) => {
    const { success, message, code } = high.createRiver(
        req.params.oceanName,
        req.params.seaName,
        req.params.riverName,
        req.params.value
    )
    endHandler(success, message, req, res, 201, code)
})