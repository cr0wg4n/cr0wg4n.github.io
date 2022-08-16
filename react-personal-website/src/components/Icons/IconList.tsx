import React from 'react';
import IconItem from './IconItem';
import loadData from '../../data/social';


const IconList: React.FC = ()=>{
    const data = loadData();
    return <>
        {data.map((item,index)=>{
            return <>
                <div key={item.url}>
                    <IconItem>
                        <a href={item.url} rel="noopener nofollow noreferrer" target={'_blank'}>
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
