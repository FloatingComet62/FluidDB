import low from './low'

function createOcean(oceanName: string) {
    const { success, message } = low.getData()
    if (!success) return { success: false, message: message as string }
    message[oceanName] = {}
    low.setData(message)
    return { success: true, message: 'Done' }
}

function createSea(oceanName: string, seaName: string) {
    const { success, message } = low.getData()
    if (!success) return { success: false, message: message as string }
    message[oceanName][seaName] = {}
    low.setData(message)
    return { success: true, message: 'Done' }
}

function createRiver(oceanName: string, seaName: string, riverName: string, value: string) {
    const { success, message } = low.getData()
    if (!success) return { success: false, message: message as string }
    message[oceanName][seaName][riverName] = value
    low.setData(message)
    return { success: true, message: 'Done' }
}

function getEveryThing() {
    const { success, message } = low.getData()
    if (!success) return { success: false, message: message as string }
    return message
}

export default {
    getEveryThing,
    createOcean,
    createSea,
    createRiver,
}