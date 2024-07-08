Usage:

    cargo run | tee output.txt

As the logging screams past your terminal, in another terminal find a block

    grep block output.txt | head -1
    Found block: "3dHta4o3UtfeNcyBTu1AcXVo2TuwbeMy2XDQoVE1P7vf"

Then curl the block id:

    curl localhost:1337/api/v1/block/3dHta4o3UtfeNcyBTu1AcXVo2TuwbeMy2XDQoVE1P7vf
    {"id":"3dHta4o3UtfeNcyBTu1AcXVo2TuwbeMy2XDQoVE1P7vf","previous_blockhash":"9GUZz6dABMkvCZk5ymBNuJQoTfwca4xdv916fEC4uMzZ","slot":281518862,"block_time":1720422738,"block_height":244135978}

Find the slot from the above output, and curl the slot:

    curl localhost:1337/api/v1/block-by-slot/281518862

Do the same for transactions and accounts:

    grep transaction output.txt | head -1
    Found transaction: "5A62giziNeSLmFVVyDZRbv54KbJLKJTJ89s2zztm9SibHfFc7H34j82wt2y21r8DRDecC1yexTvhJWGdUje9qM5x"

    curl localhost:1337/api/v1/transaction/5A62giziNeSLmFVVyDZRbv54KbJLKJTJ89s2zztm9SibHfFc7H34j82wt2y21r8DRDecC1yexTvhJWGdUje9qM5x
    {"id":"5A62giziNeSLmFVVyDZRbv54KbJLKJTJ89s2zztm9SibHfFc7H34j82wt2y21r8DRDecC1yexTvhJWGdUje9qM5x","err":"InstructionError","recent_blockhash":"9vafKstYZ63TMySsoZvSmAXgcLuZnkVu8DbuYhuo6Vc9","signatures":["5A62giziNeSLmFVVyDZRbv54KbJLKJTJ89s2zztm9SibHfFc7H34j82wt2y21r8DRDecC1yexTvhJWGdUje9qM5x"],"accounts":["3dNzZLsbYP33H1Pcu5tht17e2DsACqZJ61DDcrjCfVPq","2Zn77yZspohsPkLP9zcWX3dxuQ69dTRNyJciVEDENJh3","4iT1VqyepwZhuiNvKxen1RGLniFX3gA3NFtMLLRtHVXA","ComputeBudget111111111111111111111111111111","SysvarS1otHashes111111111111111111111111111","cookr8CThnfEQZvvrB6zhh5K4X8XNkPjJi4uUDtkBuG","3amHhT6cLgvfjKWbka6DYjs9zS5pLFnmYw1g8C6DPa4x"],"balances":[2220021680,1057920,1559040,1,143487360,1141440,1614720],"fees":5000}

    grep account output.txt | head -1
    Found account: "3dNzZLsbYP33H1Pcu5tht17e2DsACqZJ61DDcrjCfVPq"

    curl localhost:1337/api/v1/account/3dNzZLsbYP33H1Pcu5tht17e2DsACqZJ61DDcrjCfVPq
    {"id":"3dNzZLsbYP33H1Pcu5tht17e2DsACqZJ61DDcrjCfVPq","balance":2219921680}

-- Alfie