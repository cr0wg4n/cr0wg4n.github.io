import React from 'react';
import IconItem from './IconItem';
import loadData from '../../data/social';
import ReactGa from 'react-ga'

const IconList: React.FC = ()=>{
    const registerEvent =  (socialName: string) => {
        ReactGa.event({
            category: `social:${String(socialName).toLowerCase()}`,
            action:'click:social',
        })
    }

    const data = loadData();
    return <>
        {data.map((item,index)=>{
            return <>
                <div key={item.url}>
                    <IconItem>
                        <a href={item.url} rel="noopener nofollow noreferrer" target={'_blank'} onClick={
                            ()=>registerEvent(item.name)
                        }>
                            {item.icon({})}
                        </a>
                    </IconItem>
                    {index!==data.length-1?<br />:null}
                </div>
            </>
        })}
    </>;
}

export default IconList; 
