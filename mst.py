from dataclasses import dataclass, field
from enum import Enum
from heapq import heappop, heappush
from itertools import count
from math import isnan

def prim_mst_edges(G, weight="weight", keys=True, data=True, ignore_nan=False):
    # minimum shouldve been added but removed for now
    #is_multigraph = G.is_multigraph()          #todo#
    nodes = set(G)
    c = count()
    
    #sign = 1 if minimum else -1 #WILL NOT DO! Just focus on minimum spanning trees
    
    while nodes:
        u = nodes.pop()
        priorityQueue = []

        
        for nbr, attr in G.adj[u].items():
            edgeWeight =attr.get(weight, 1)
            if isnan(edgeWeight):
                if ignore_nan:
                    continue
                msg = f"NaN found as an edge weight. Edge {(u, nbr, attr)}"
                raise ValueError(msg)
            heappush(priorityQueue, (edgeWeight, next(c), u, nbr, attr))
            
        while nodes and priorityQueue:
            # if is_multigraph:
            #     W, _, u,attr, k, d = heappop(frontier)
            # else:
            
            W, _, u, nbr,attr = heappop(priorityQueue)
            
            if nbr in visited or nbr not in nodes:
                continue
            '''
            # if is_multigraph and keys:
            #     if data:
            #         yield u, nbr, k,attr
            #     else:
            #         yield u, nbr, k
            # else:
            '''
            if data:
                yield u, nbr,attr
            else:
                yield u, nbr
                
            # Update Frontier
            visited.add(nbr)
            nodes.discard(nbr)
            '''
            # if is_multigraph:
            #     for w, keydict in G.adjattr].items():
            #         if w in visited:
            #             continue
            #         for k2, d2 in keydict.items():
            #             new_weight = d2.get(weight, 1) * sign
            #             if isnan(new_weight):
            #                 if ignore_nan:
            #                     continue
            #                 msg = f"NaN found as an edge weight. Edge {attr, w, k2, d2)}"
            #                 raise ValueError(msg)
            #             heappush(frontier, (new_weight, next(c),attr, w, k2, d2))
            # else:'''
            for secondNbr, v2 in G.adj[nbr].items():
                if secondNbr in visited:
                    continue
                new_weight = v2.get(weight, 1)
                if isnan(new_weight):
                    if ignore_nan:
                        continue
                    msg = f"NaN found as an edge weight. Edge {(nbr, secondNbr, v2)}"
                    raise ValueError(msg)
                heappush(priorityQueue, (new_weight, next(c), nbr, secondNbr, v2))