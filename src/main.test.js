beforeAll(async function () {
  // NOTE: nearlib and nearConfig are made available by near-cli/test_environment
  const near = await nearlib.connect(nearConfig)
  window.accountId = nearConfig.contractName
  // console.log(window.accountId)
  window.contract = await near.loadContract(nearConfig.contractName, {
    viewMethods: ['get_message'],
    changeMethods: ['clain_reward'],
    sender: window.accountId
  })
})

test('get_message', async () => {
  const message = await window.contract.get_message({ account_id: window.accountId })
  expect(message).toEqual('Hello')
})

test('clain_reward', async () => {
  const clain_reward = await window.contract.clain_reward();
  expect(clain_reward).toEqual('') 
})
