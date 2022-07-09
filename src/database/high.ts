import low from './low'
import { HighGetResponse, HighResponse } from './types'

function getEveryThing(): HighResponse {
    const { success, message } = low.getData()
    if (!success) return { success: false, message, code: 2 }
    return { success: true, message, code: 0 }
}


function createOcean(oceanName: string): HighResponse {
    const { success, message } = low.getData()
    if (!success) return { success: false, message, code: 2 }
    message[oceanName] = {}
    
    const { success: setSuccess, message: setMessage } = low.setData(message)
    if (!setSuccess) return { success: false, message: setMessage, code: 2 }
    return { success: true, message, code: 0 }
}

function createSea(oceanName: string, seaName: string): HighResponse {
    const { success, message } = low.getData()
    if (!success) return { success: false, message, code: 2 }
    if (!message[oceanName]) return { success: false, message: `Ocean ${oceanName} does not exist`, code: 1 }
    message[oceanName][seaName] = {}

    const { success: setSuccess, message: setMessage } = low.setData(message)
    if (!setSuccess) return { success: false, message: setMessage, code: 2 }
    return { success: true, message, code: 0 }
}

function createRiver(
    oceanName: string,
    seaName: string,
    riverName: string,
    value: string
): HighResponse {
    const { success, message } = low.getData()
    if (!success) return { success: false, message, code: 2 }
    if (!message[oceanName]) return { success: false, message: `Ocean ${oceanName} does not exist`, code: 1 }
    if (!message[oceanName][seaName]) return { success: false, message: `Sea ${seaName} does not exist`, code: 1 }
    message[oceanName][seaName][riverName] = value

    const { success: setSuccess, message: setMessage } = low.setData(message)
    if (!setSuccess) return { success: false, message: setMessage, code: 2 }
    return { success: true, message, code: 0 }
}


function getOcean(oceanName: string): HighGetResponse {
    const { success, message } = low.getData()
    if (!success) return { success: false, message, code: 2 }
    if (!message[oceanName]) return { success: false, message: `Ocean ${oceanName} does not exist`, code: 1 }
    return { success: true, message: message[oceanName], code: 0 }
}

function getSea(oceanName: string, seaName: string): HighGetResponse {
    const { success, message } = low.getData()
    if (!success) return { success: false, message, code: 2 }
    if (!message[oceanName]) return { success: false, message: `Ocean ${oceanName} does not exist`, code: 1 }
    if (!message[oceanName][seaName]) return { success: false, message: `Sea ${seaName} does not exist`, code: 1 }
    return { success: true, message: message[oceanName][seaName], code: 0 }
}

function getRiver(oceanName: string, seaName: string, riverName: string): HighGetResponse {
    const { success, message } = low.getData()
    if (!success) return { success: false, message, code: 2 }
    if (!message[oceanName]) return { success: false, message: `Ocean ${oceanName} does not exist`, code: 1 }
    if (!message[oceanName][seaName]) return { success: false, message: `Sea ${seaName} does not exist`, code: 1 }
    if (!message[oceanName][seaName][riverName]) return { success: false, message: `River ${riverName} does not exist`, code: 1 }
    return { success: true, message: message[oceanName][seaName][riverName], code: 0 }
}

export default {
    getEveryThing,

    createOcean,
    createSea,
    createRiver,

    getOcean,
    getSea,
    getRiver,
}