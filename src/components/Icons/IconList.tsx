import React from 'react';
import IconItem from './IconItem';
import loadData from '../../data/social';
import ReactGa from 'react-ga4'

const IconList: React.FC = ()=>{
    const registerEvent =  (socialName: string) => {
        ReactGa.event({
            category: `social`,
            action:`click:${String(socialName).toLowerCase()}`,
        })
    }

    const data = loadData();
    return <>
        {data.map((item,index)=>{
            return <>
                <div key={item.url}>
                    <IconItem>
                        <a href={item.url} aria-label={item.name} rel="noopener nofollow noreferrer" target={'_blank'} onClick={
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
